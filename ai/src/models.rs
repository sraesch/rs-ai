use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Architecture {
    pub modality: String,
    pub input_modalities: Vec<String>,
    pub output_modalities: Vec<String>,
    pub tokenizer: String,
    pub instruct_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pricing {
    pub prompt: String,
    pub completion: String,
    pub request: Option<String>,
    pub image: Option<String>,
    pub web_search: Option<String>,
    pub internal_reasoning: Option<String>,
    pub input_cache_read: Option<String>,
    pub input_cache_write: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopProvider {
    pub context_length: Option<u64>,
    pub max_completion_tokens: Option<u64>,
    pub is_moderated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub hugging_face_id: Option<String>,
    pub name: String,
    pub created: u64,
    pub description: String,
    pub context_length: u64,
    pub architecture: Architecture,
    pub pricing: Pricing,
    pub top_provider: TopProvider,
    pub per_request_limits: Option<HashMap<String, String>>,
    pub supported_parameters: Vec<String>,
}

/// Represents the list of models available in the API.
#[derive(Debug, Serialize, Deserialize)]
pub struct Models {
    #[serde(rename = "data")]
    pub models: Vec<Entry>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_models_deserialization() {
        let json_data = include_str!("../test_data/models.json");
        let data: Models = serde_json::from_str(json_data).unwrap();
        assert!(!data.models.is_empty(), "Failed to deserialize models");
    }
}
