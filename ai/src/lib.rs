mod error;
mod models;
mod tools;

pub mod json_types;

pub use error::*;
use json_types::ResponseFormat;
pub use json_types::{
    ChatCompletionResponse, Choice, JsonFunctionInfo, JsonSchemaDescription, JsonTool, Message,
    ToolChoice, Usage,
};
pub use models::*;
use schemars::JsonSchema;
pub use tools::*;

use log::{debug, log_enabled, trace};
use reqwest::{StatusCode, Url};

/// A client for interacting with the LLM API.
pub struct Client {
    api_key: String,
    api_url: Url,
    client: reqwest::Client,
    models: Option<LLMModels>,
}

impl Client {
    /// Creates a new `Client` instance with the given API key and URL.
    ///
    /// # Arguments
    /// * `api_key` - The API key to authenticate requests.
    /// * `api_url` - The base URL for the API.
    pub fn new(api_key: String, api_url: Url) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| {
                log::error!("Failed to create HTTP client: {}", e);
                Error::HTTPError(Box::new(e))
            })?;

        Ok(Self {
            api_key,
            api_url,
            client,
            models: None,
        })
    }

    /// Returns a reference onto the models.
    /// If the models are not loaded, it fetches them from the API.
    pub async fn get_models(&mut self) -> Result<&LLMModels> {
        let not_loaded = self.models.is_none();

        // If models are not loaded, fetch them from the API
        if not_loaded {
            let url = self.api_url.join("models").unwrap();
            debug!("Request URL: {}", url);
            let response = self.client.get(url).send().await.map_err(|e| {
                log::error!("Request failed: {}", e);
                Error::HTTPError(Box::new(e))
            })?;

            if response.status().is_success() {
                let response_body = response.text().await.map_err(|e| {
                    log::error!("Failed to read response body: {}", e);
                    Error::HTTPError(Box::new(e))
                })?;

                debug!("Response body: {}", response_body);
                let response = serde_json::from_str::<JsonModels>(&response_body).map_err(|e| {
                    log::error!("Failed to parse response: {}", e);
                    Error::Deserialization(e.to_string())
                })?;

                self.models = Some(LLMModels::new(response));

                Ok(self.models.as_ref().unwrap())
            } else {
                let status = response.status();
                let response_message = response.text().await.unwrap_or_default();
                log::error!(
                    "Request failed (status={}): Message={}",
                    status,
                    response_message
                );
                Err(Error::HTTPErrorWithStatusCode(status))
            }
        } else {
            // If models are already loaded, return them
            Ok(self.models.as_ref().unwrap())
        }
    }

    /// Sends a chat completion request to the API.
    /// Returns a vector of messages as the response.
    ///
    /// # Arguments
    /// * `parameter` - The parameter for the chat completion request.
    pub async fn chat_completion(
        &self,
        parameter: &ChatCompletionParameter<'_>,
    ) -> Result<Vec<Choice>> {
        let mut request_body = json_types::ChatCompletionRequest::new(
            parameter.model.as_str(),
            parameter.messages.as_ref(),
        );

        request_body.response_format = parameter.response_format.clone();
        request_body.tools = parameter.tools.as_ref();
        request_body.tool_choice = parameter.tool_choice.clone();

        // create the url for the request
        let url = self.api_url.join("chat/completions").unwrap();
        debug!("Request URL: {}", url);

        // if log level is set to trace, print the request body
        if log_enabled!(log::Level::Trace) {
            let request_body_str = serde_json::to_string_pretty(&request_body).unwrap();
            trace!("Request body: {}", request_body_str);
        }

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                log::error!("Request failed: {}", e);
                Error::HTTPError(Box::new(e))
            })?;

        if response.status().is_success() {
            let response_body = response.text().await.map_err(|e| {
                log::error!("Failed to read response body: {}", e);
                Error::HTTPError(Box::new(e))
            })?;

            debug!("Response body: {}", response_body);
            let response =
                serde_json::from_str::<ChatCompletionResponse>(&response_body).map_err(|e| {
                    log::error!("Failed to parse response: {}", e);
                    Error::Deserialization(e.to_string())
                })?;

            Ok(response.choices)
        } else {
            if response.status() == StatusCode::BAD_REQUEST {
                let response_body = response.text().await.map_err(|e| {
                    log::error!("Failed to read response body: {}", e);
                    Error::HTTPError(Box::new(e))
                })?;

                log::error!("Response body: {}", response_body);
                return Err(Error::BadRequest(response_body));
            }

            let status = response.status();
            let response_message = response.text().await.unwrap_or_default();
            log::error!(
                "Request failed (status={}): Message={}",
                status,
                response_message
            );
            Err(Error::HTTPErrorWithStatusCode(status))
        }
    }
}

/// The parameter for a a chat completion request.
pub struct ChatCompletionParameter<'a> {
    model: String,
    messages: Vec<Message>,
    response_format: Option<ResponseFormat<'a>>,
    tools: Vec<JsonTool>,
    tool_choice: Option<ToolChoice>,
}

impl<'a> ChatCompletionParameter<'a> {
    /// Creates a new `ChatCompletionRequest` with the given model and messages.
    ///
    /// # Arguments
    /// * `model` - The model to use for the chat completion.
    /// * `messages` - A slice of messages to send in the request.
    pub fn new(model: String, messages: Vec<Message>) -> Self {
        Self {
            model,
            messages,
            response_format: None,
            tools: Vec::new(),
            tool_choice: None,
        }
    }

    /// Sets the response format for the chat completion request.
    ///
    /// # Arguments
    /// * `response_format` - The response format to use.
    pub fn set_response_format(&mut self, response_format: ResponseFormat<'a>) {
        self.response_format = Some(response_format);
    }

    /// Appends another message to the request.
    ///
    /// # Arguments
    /// * `message` - The message to append.
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Appends a tool to the request.
    ///
    /// # Arguments
    /// * `tool` - The tool to append.
    pub fn add_tool<P: JsonSchema>(&mut self, tool: Tool<P>) {
        let json_tool = tool.into_json();
        self.tools.push(json_tool);
    }

    /// Sets the tool choice for the request.
    ///
    /// # Arguments
    /// * `tool_choice` - The tool choice to set.
    pub fn set_tool_choice(&mut self, tool_choice: ToolChoice) -> Result<()> {
        if let ToolChoice::Function(f) = &tool_choice {
            // check if the specified function is in the tools
            if !self
                .tools
                .iter()
                .any(|tool| tool.function.name == f.function.name)
            {
                return Err(Error::ToolNotFound(f.function.name.clone()));
            }
        }

        self.tool_choice = Some(tool_choice);

        Ok(())
    }
}
