use ai::{Client, JsonSchemaDescription, Message, json_types::ResponseFormat};
use log::{LevelFilter, info};
use schemars::{JsonSchema, schema_for};
use serde::Deserialize;

use std::io::Write;

/// Initializes the program logging
///
/// # Arguments
/// * `filter` - The log level filter, i.e., the minimum log level to be logged.
fn initialize_logging(filter: LevelFilter) {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(filter)
        .init();
}

#[derive(JsonSchema, Debug, PartialEq, Deserialize)]
#[schemars(deny_unknown_fields)]
struct Country {
    name: String,
    capital: String,
    population: u64,
}

#[derive(JsonSchema, Debug, PartialEq, Deserialize)]
#[schemars(deny_unknown_fields)]
struct Countries {
    countries: Vec<Country>,
}

#[tokio::main]
async fn main() {
    initialize_logging(LevelFilter::Info);

    // check for API_KEY in environment variables
    // Get the environment variable API_KEY
    info!("Load API_KEY...");
    let api_key = std::env::var("API_KEY").unwrap_or_default();
    info!("Load API_KEY...DONE");
    if api_key.is_empty() {
        panic!("API_KEY environment variable is not set");
    }

    // Load model
    info!("Load model from environment variable LLM_MODEL...");
    let mut model = std::env::var("LLM_MODEL").unwrap_or_default();
    if model.is_empty() {
        info!("LLM_MODEL environment variable is not set, using default model");
        model = "openai/gpt-4.1".into();
    }

    info!("Model={}", model);
    info!("Load model...DONE");

    // create client with
    info!("Create client...");
    let client = Client::new(api_key, "https://openrouter.ai/api/v1/".parse().unwrap()).unwrap();
    info!("Create client...DONE");

    // Use the client
    info!("Use client...");
    let message = Message {
        role: "user".to_string(),
        tool_call_id: String::new(),
        content: "Name a few european countries.".to_string(),
        tool_calls: vec![],
    };

    let mut prompt = ai::ChatCompletionParameter::new(model.clone(), vec![message]);

    // set the schema for the response
    let schema = schema_for!(Countries);

    let json_schema = JsonSchemaDescription {
        name: "Countries".to_string(),
        strict: true,
        schema,
    };

    prompt.set_response_format(ResponseFormat {
        json_schema: Some(&json_schema),
        schema_type: "json_schema",
    });

    let response = client.chat_completion(&prompt).await.unwrap();

    for choice in response {
        println!("Raw Response is: {}", choice.message.content);
        let countries: Countries = serde_json::from_str(&choice.message.content).unwrap();
        println!("JSON Response: {:?}", countries);
    }
    info!("Use client...DONE");
}
