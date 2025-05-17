use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonArchitecture {
    pub modality: String,
    pub input_modalities: Vec<String>,
    pub output_modalities: Vec<String>,
    pub tokenizer: String,
    pub instruct_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonPricing {
    pub prompt: String,
    pub completion: String,
    pub request: Option<String>,
    pub image: Option<String>,
    pub web_search: Option<String>,
    pub internal_reasoning: Option<String>,
    pub input_cache_read: Option<String>,
    pub input_cache_write: Option<String>,
}

impl std::fmt::Display for JsonPricing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Prompt: {}, Completion: {}",
            self.prompt, self.completion,
        )?;

        if let Some(request) = &self.request {
            write!(f, ", Request: {}", request)?;
        }

        if let Some(image) = &self.image {
            write!(f, ", Image: {}", image)?;
        }

        if let Some(web_search) = &self.web_search {
            write!(f, ", Web Search: {}", web_search)?;
        }

        if let Some(internal_reasoning) = &self.internal_reasoning {
            write!(f, ", Internal Reasoning: {}", internal_reasoning)?;
        }

        if let Some(input_cache_read) = &self.input_cache_read {
            write!(f, ", Input Cache Read: {}", input_cache_read)?;
        }

        if let Some(input_cache_write) = &self.input_cache_write {
            write!(f, ", Input Cache Write: {}", input_cache_write)?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonTopProvider {
    pub context_length: Option<u64>,
    pub max_completion_tokens: Option<u64>,
    pub is_moderated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMModel {
    pub id: String,
    pub hugging_face_id: Option<String>,
    pub name: String,
    pub created: u64,
    pub description: String,
    pub context_length: u64,
    pub architecture: JsonArchitecture,
    pub pricing: JsonPricing,
    pub top_provider: JsonTopProvider,
    pub per_request_limits: Option<HashMap<String, String>>,
    pub supported_parameters: HashSet<String>,
}

/// Represents the list of models available in the API.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonModels {
    #[serde(rename = "data")]
    pub models: Vec<LLMModel>,
}

/// The LLMModels to give information about the available models.
pub struct LLMModels {
    models: JsonModels,
}

impl LLMModels {
    /// Creates a new `LLMModels` instance with the given models.
    ///
    /// # Arguments
    /// * `models` - The models to use for the LLMModels.
    pub fn new(models: JsonModels) -> Self {
        Self { models }
    }

    /// Returns the list of models.
    pub fn get_models(&self) -> &[LLMModel] {
        &self.models.models
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_models_deserialization() {
        let json_data = include_str!("../test_data/models.json");
        let data: JsonModels = serde_json::from_str(json_data).unwrap();
        assert!(!data.models.is_empty(), "Failed to deserialize models");
    }
}
