mod options;

use ai::Message;
use anyhow::Result;
use clap::Parser as _;
use dotenv::dotenv;
use log::{LevelFilter, error, info};
use options::{Commands, Options};
use std::io::Write as _;

/// Parses the program arguments and returns None, if no arguments were provided and Some otherwise.
fn parse_args() -> Result<Options> {
    let options = Options::parse();
    Ok(options)
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
    let mut client = ai::Client::new(api_key, options.api_endpoint.parse()?)?;
    info!("Create client...Ok");

    match options.command {
        Commands::Models(models_options) => {
            command_list_models(&mut client, &models_options).await?;
        }
        Commands::Prompt(prompt_options) => {
            command_prompt(&mut client, &prompt_options).await?;
        }
    }

    Ok(())
}

/// The command to list the models available in the API
///
/// # Arguments
/// * `client` - The client to use for the API requests.
/// * `models_options` - The options for the command.
async fn command_list_models(
    client: &mut ai::Client,
    models_options: &options::QueryModelsArguments,
) -> Result<()> {
    let models = client.get_models().await?;

    for model in models.get_models() {
        if let Some(search_string) = &models_options.search_string {
            if !model.name.to_lowercase().contains(search_string) {
                continue;
            }
        }

        if models_options.structured_output
            && !model.supported_parameters.contains("structured_outputs")
        {
            continue;
        }

        println!("Model: {}", model.name);
        println!("  ID: {}", model.id);
        println!("  Context length: {}", model.context_length);

        if models_options.show_pricing {
            println!("  Pricing: {}", model.pricing);
        }
    }

    Ok(())
}

async fn command_prompt(
    client: &mut ai::Client,
    prompt_options: &options::PromptArguments,
) -> Result<()> {
    let prompt = Message {
        role: "user".to_string(),
        content: prompt_options.prompt.clone(),
        tool_calls: vec![],
    };

    let response = client
        .chat_completion(&prompt_options.model, &[prompt])
        .await?;

    for choice in response {
        println!("Response: {}", choice.message.content);
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
