mod options;

use ai::{JsonSchemaDescription, Message};
use anyhow::Result;
use clap::Parser as _;
use dotenv::dotenv;
use log::{LevelFilter, error, info};
use options::Options;
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use std::io::Write as _;

/// Parses the program arguments and returns None, if no arguments were provided and Some otherwise.
fn parse_args() -> Result<Options> {
    let options = Options::parse();
    Ok(options)
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
#[schemars(deny_unknown_fields)]
struct Llm {
    /// The name of the LLM
    pub name: String,

    /// The company or entity that is responsible for the LLM
    pub company: String,

    /// A short description of the LLM
    pub description: String,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
#[schemars(deny_unknown_fields)]
struct LLMList {
    /// A list of LLMs
    pub list: Vec<Llm>,
}

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

/// Runs the program.
async fn run_program() -> Result<()> {
    let options = parse_args()?;
    initialize_logging(LevelFilter::from(options.log_level));

    // Get the environment variable API_KEY
    info!("Load API_KEY...");

    if let Err(err) = dotenv() {
        anyhow::bail!("Failed to load .env file: {}", err);
    }

    let api_key = match std::env::var("API_KEY") {
        Ok(api_key) => api_key,
        Err(err) => {
            anyhow::bail!("Failed to get API_KEY: {}", err);
        }
    };

    info!("Load API_KEY...Ok");

    info!("Options:");
    options.dump_to_log();
    info!("-------");

    info!("Create client...");
    let client = ai::Client::new(api_key, options.api_endpoint.parse()?);
    info!("Create client...Ok");

    let schema = schema_for!(LLMList);

    let json_schema = JsonSchemaDescription {
        name: "LLM".to_string(),
        strict: true,
        schema,
    };

    println!(
        "JSON Schema: {}",
        serde_json::to_string_pretty(&json_schema)?
    );

    let choices = client
        .chat_completion_structured(
            "openai/gpt-4.1",
            &[Message {
                role: "user".to_string(),
                content: "Name a few LLMs".to_string(),
            }],
            &json_schema,
        )
        .await?;

    info!("Response:");
    for choice in choices.iter() {
        let llm: LLMList = serde_json::from_str(&choice.message.content).unwrap();
        for (index, l) in llm.list.iter().enumerate() {
            info!("{}: {:?}", index, l);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    match run_program().await {
        Ok(()) => {
            info!("SUCCESS");
        }
        Err(err) => {
            error!("Error: {}", err);
            error!("FAILED");

            std::process::exit(-1);
        }
    }
}
