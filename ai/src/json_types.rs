use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};

/// The request body used in the chat completion API
#[derive(Serialize, Debug)]
pub struct ChatCompletionRequest<'a, 'b, 'c, 'd> {
    pub model: &'a str,
    pub messages: &'b [Message],

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub tools: &'d [JsonTool],

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat<'c>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ResponseFormat<'a> {
    #[serde(rename = "type")]
    pub schema_type: &'static str,
    pub json_schema: Option<&'a JsonSchemaDescription>,
}

#[derive(Serialize, Debug)]
pub struct JsonSchemaDescription {
    pub name: String,
    pub strict: bool,
    pub schema: RootSchema,
}

/// Represents the response format for the chat completion request.
const EMPTY_TOOLS: [JsonTool; 0] = [];

impl<'a, 'b> ChatCompletionRequest<'a, 'b, '_, '_> {
    /// Creates a new `ChatCompletionRequest` with the given model and messages.
    ///
    /// # Arguments
    /// * `model` - The model to use for the chat completion.
    /// * `messages` - A slice of messages to send in the request.
    pub fn new(model: &'a str, messages: &'b [Message]) -> Self {
        Self {
            model,
            messages,
            tool_choice: None,
            response_format: None,
            tools: &EMPTY_TOOLS,
        }
    }
}

/// Represents the response from the chat completion API.
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionResponse {
    pub id: String,

    #[serde(default)]
    pub provider: String,

    #[serde(default)]
    pub model: String,

    #[serde(default)]
    pub object: String,

    #[serde(default)]
    pub system_fingerprint: Option<String>,

    pub usage: Usage,

    pub created: i64,
    pub choices: Vec<Choice>,
}

/// Represents the usage information in the chat completion response.
#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

/// Represents a message in the chat completion request/response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tool_call_id: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<JsonToolCall>,
}

/// Represents a tool call in the message.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonToolCall {
    pub index: i64,
    pub id: String,
    pub r#type: String,

    #[serde(rename = "function")]
    pub function_call: JsonFunctionCall,
}

/// Represents a function call in the tool call.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonFunctionCall {
    pub name: String,
    pub arguments: String,
}

/// Represents a single choice in the chat completion response.
#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: i64,
    pub finish_reason: String,
    pub native_finish_reason: String,
    pub message: Message,
}

/// Represents a tool used in the chat completion request.
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonTool {
    /// The type of tool. Must be "function".
    #[serde(rename = "type")]
    pub tool_type: String,

    /// The function definition of the tool.
    pub function: JsonFunctionInfo,
}

/// The function definition for a tool.
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonFunctionInfo {
    /// The name of the function.
    pub name: String,

    /// The description of the function.
    pub description: String,

    /// If strict is true, the function must be called with all required parameters.
    pub strict: bool,

    /// The parameters for the function.
    pub parameters: RootSchema,
}

/// Represents the choice of tool to be used in the chat completion request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ToolChoice {
    #[serde(rename = "auto")]
    Auto,

    #[serde(rename = "required")]
    Required,

    #[serde(untagged)]
    Function(ToolChoiceFunction),
}

/// Represents a function choice in the tool.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolChoiceFunction {
    /// Must be "function".
    pub r#type: String,

    /// The function definition of the tool.
    pub function: ToolChoiceFunctionDesc,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolChoiceFunctionDesc {
    /// The name of the function.
    pub name: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decoding_chat_completion_response() {
        let json = r#"
        {
            "id": "gen-1747167300-Qc7IgPZUPoopdSABk5KA",
            "provider": "OpenAI",
            "model": "openai/gpt-3.5-turbo",
            "object": "chat.completion",
            "created": 1747167300,
            "choices": [
                {
                "logprobs": null,
                "finish_reason": "stop",
                "native_finish_reason": "stop",
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Hello! I am an AI assistant, so I don't have feelings, but I'm here and ready to help you with anything you need. How can I assist you today?",
                    "refusal": null,
                    "reasoning": null
                }
                }
            ],
            "system_fingerprint": null,
            "usage": {
                "prompt_tokens": 13,
                "completion_tokens": 37,
                "total_tokens": 50,
                "prompt_tokens_details": { "cached_tokens": 0 },
                "completion_tokens_details": { "reasoning_tokens": 0 }
            }
            }
        "#;

        let response: ChatCompletionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "gen-1747167300-Qc7IgPZUPoopdSABk5KA");
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct MyStruct {
        pub tool_choice: ToolChoice,
    }

    #[test]
    fn test_encoding_tool_choice() {
        let tool_choice = ToolChoice::Auto;
        let json = serde_json::to_string(&MyStruct { tool_choice }).unwrap();
        assert_eq!(json, r#"{"tool_choice":"auto"}"#,);

        let tool_choice = ToolChoice::Required;
        let json = serde_json::to_string(&MyStruct { tool_choice }).unwrap();
        assert_eq!(json, r#"{"tool_choice":"required"}"#,);
    }

    #[test]
    fn test_encoding_tool_choice_function() {
        let tool_choice = ToolChoice::Function(ToolChoiceFunction {
            r#type: "function".to_string(),
            function: ToolChoiceFunctionDesc {
                name: "get_weather".to_string(),
            },
        });

        let json = serde_json::to_string(&MyStruct { tool_choice }).unwrap();
        assert_eq!(
            json,
            r#"{"tool_choice":{"type":"function","function":{"name":"get_weather"}}}"#,
        );
    }
}
