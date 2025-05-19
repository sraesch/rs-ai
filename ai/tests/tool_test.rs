use std::collections::BTreeSet;

use ai::Tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct HelperJsonTool {
    /// The type of tool. Must be "function".
    #[serde(rename = "type")]
    pub tool_type: String,

    /// The function definition of the tool.
    pub function: HelperJsonFunctionInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct HelperJsonFunctionInfo {
    /// The name of the function.
    pub name: String,

    /// The description of the function.
    pub description: String,

    /// The parameters for the function.
    pub parameters: HelperJsonFunctionParameters,

    /// Whether the function is strict.
    pub strict: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct HelperJsonFunctionParameters {
    /// The type of the parameters.
    #[serde(rename = "type")]
    pub param_type: String,

    /// The properties of the parameters.
    pub properties: std::collections::BTreeMap<String, HelperJsonFunctionProperty>,

    /// The required properties.
    pub required: BTreeSet<String>,

    /// Whether additional properties are allowed.
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct HelperJsonFunctionProperty {
    /// The type of the property.
    #[serde(rename = "type")]
    pub prop_type: String,

    /// The description of the property.
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct WeatherParameter {
    /// City and country e.g. Bogotá, Colombia
    pub location: String,
}

#[test]
fn test_tool() {
    // load reference json
    let weather_tool_str = include_str!("../test_data/weather_tool.json");
    let reference: HelperJsonTool = serde_json::from_str(weather_tool_str).unwrap();

    // create the schema for the weather tool
    let tool = Tool::<WeatherParameter>::new(
        "get_weather".to_string(),
        "Get current temperature for a given location.".to_string(),
    )
    .into_json();

    // serialize the tool to JSON
    let tool_json = serde_json::to_string_pretty(&tool).unwrap();

    // deserialize the JSON back to the struct
    let deserialized_tool: HelperJsonTool = serde_json::from_str(&tool_json).unwrap();

    assert_eq!(reference, deserialized_tool);
}
