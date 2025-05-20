use clap::{Args, Parser, Subcommand, ValueEnum};
use log::{LevelFilter, info};

/// Workaround for parsing the different log level
#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => LevelFilter::Trace,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Error => LevelFilter::Error,
        }
    }
}

/// CLI interface for determining the pixel contribution of the geometry from all views.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Options {
    /// The log level
    #[arg(short, value_enum, long, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,

    /// The API endpoint to use
    #[arg(short, long, default_value = "https://openrouter.ai/api/v1/")]
    pub api_endpoint: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Query the LLM models available in the API
    Models(QueryModelsArguments),

    /// Giving a prompt to the LLM
    Prompt(PromptArguments),

    /// A simple test command to check if the tool API is working
    Weather(WeatherArguments),
}

#[derive(Args, Debug, Clone)]
pub struct QueryModelsArguments {
    /// Optional search string to filter the models
    #[arg(short, long)]
    pub search_string: Option<String>,

    /// Filter for models that support structured output
    #[arg(short = 'c', long, default_value_t = false)]
    pub structured_output: bool,

    /// Filter for models that support function calling
    #[arg(short = 'f', long, default_value_t = false)]
    pub function_calling: bool,

    /// Show the pricing information for the models
    #[arg(short = 'p', long, default_value_t = false)]
    pub show_pricing: bool,
}

#[derive(Args, Debug, Clone)]
pub struct PromptArguments {
    /// The prompt to send to the LLM
    #[arg(short, long)]
    pub prompt: String,

    /// The model to use for the prompt
    #[arg(short, long)]
    pub model: String,
}

#[derive(Args, Debug, Clone)]
pub struct WeatherArguments {
    /// The model to use for the prompt
    #[arg(short, long)]
    pub model: String,
}

impl Options {
    /// Dumps the options to the log.
    pub fn dump_to_log(&self) {
        info!("log_level: {:?}", self.log_level);
        info!("api_endpoint: {:?}", self.api_endpoint);
    }
}
