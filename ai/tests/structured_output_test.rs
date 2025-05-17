use std::collections::BTreeSet;

use ai::{JsonSchemaDescription, json_types::ResponseFormat};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize, Debug, PartialEq)]
#[schemars(deny_unknown_fields)]
struct Weather {
    /// City or location name
    pub location: String,

    /// Temperature in Celsius
    pub temperature: f32,

    /// Weather conditions description
    pub conditions: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct HelperStruct {
    pub r#type: String,

    pub json_schema: HelperJsonSchema,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct HelperJsonSchema {
    pub name: String,
    pub strict: bool,
    pub schema: HelperJsonSchemaDescription,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct HelperJsonSchemaDescription {
    pub r#type: String,
    pub properties: std::collections::BTreeMap<String, HelperJsonSchemaProperty>,
    pub required: BTreeSet<String>,

    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct HelperJsonSchemaProperty {
    pub r#type: String,
    pub description: String,
}

const REFERENCE: &str = r#"{
    "type": "json_schema",
    "json_schema": {
      "name": "weather",
      "strict": true,
      "schema": {
        "type": "object",
        "properties": {
          "location": {
            "type": "string",
            "description": "City or location name"
          },
          "temperature": {
            "type": "number",
            "description": "Temperature in Celsius"
          },
          "conditions": {
            "type": "string",
            "description": "Weather conditions description"
          }
        },
        "required": ["location", "temperature", "conditions"],
        "additionalProperties": false
      }
    }
  }"#;

#[test]
fn test_schema() {
    // load reference json
    let reference: HelperStruct = serde_json::from_str(REFERENCE).unwrap();

    // define JSON schema using the `schemars` crate
    let schema = schema_for!(Weather);
    let json_schema = JsonSchemaDescription {
        name: "weather".to_string(),
        strict: true,
        schema,
    };

    let json_schema = ResponseFormat {
        schema_type: "json_schema",
        json_schema: Some(&json_schema),
    };

    // create string representation of the JSON schema
    let json_schema_str = serde_json::to_string_pretty(&json_schema).unwrap();
    println!("JSON Schema: {}", json_schema_str);

    // parse the JSON schema string into a `HelperJsonSchemaDescription` struct as well
    let parsed_json_schema: HelperStruct = serde_json::from_str(&json_schema_str).unwrap();

    assert_eq!(
        parsed_json_schema, reference,
        "Parsed JSON schema does not match the reference"
    );
}
