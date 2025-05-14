mod error;
mod json_types;

pub use error::*;
pub use json_types::{ChatCompletionResponse, Choice, Message, Usage};
use log::debug;
use reqwest::Url;

pub struct Client {
    pub api_key: String,
    pub api_url: Url,
}

impl Client {
    /// Creates a new `Client` instance with the given API key and URL.
    ///
    /// # Arguments
    /// * `api_key` - The API key to authenticate requests.
    /// * `api_url` - The base URL for the API.
    pub fn new(api_key: String, api_url: Url) -> Self {
        Self { api_key, api_url }
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
