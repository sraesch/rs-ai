mod error;
mod models;

pub mod json_types;

pub use error::*;
use json_types::ResponseFormat;
pub use json_types::{ChatCompletionResponse, Choice, JsonSchemaDescription, Message, Usage};
pub use models::*;

use log::debug;
use reqwest::Url;

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
                log::error!("Request failed with status: {}", response.status());
                Err(Error::HTTPErrorWithStatusCode(response.status()))
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
    /// * `model` - The model to use for the chat completion.
    /// * `messages` - A slice of messages to send in the request.
    pub async fn chat_completion(&self, model: &str, messages: &[Message]) -> Result<Vec<Choice>> {
        let request_body = json_types::ChatCompletionRequest::new(model, messages);

        let url = self.api_url.join("chat/completions").unwrap();
        debug!("Request URL: {}", url);
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
            log::error!("Request failed with status: {}", response.status());
            Err(Error::HTTPErrorWithStatusCode(response.status()))
        }
    }

    /// Sends a chat completion request to the API.
    /// Returns a vector of messages as the response.
    ///
    /// # Arguments
    /// * `model` - The model to use for the chat completion.
    /// * `messages` - A slice of messages to send in the request.
    /// * `json_schema`- The JSON schema to use for the response format.
    pub async fn chat_completion_structured(
        &self,
        model: &str,
        messages: &[Message],
        json_schema: &JsonSchemaDescription,
    ) -> Result<Vec<Choice>> {
        let mut request_body = json_types::ChatCompletionRequest::new(model, messages);
        request_body.response_format = Some(ResponseFormat {
            schema_type: "json_schema",
            json_schema: Some(json_schema),
        });

        let url = self.api_url.join("chat/completions").unwrap();
        debug!("Request URL: {}", url);
        let client = reqwest::Client::new();
        let response = client
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
            log::error!("Request failed with status: {}", response.status());
            Err(Error::HTTPErrorWithStatusCode(response.status()))
        }
    }
}
