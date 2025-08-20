use std::marker::PhantomData;

use schemars::Schema;
use schemars::transform::AddNullable;
use schemars::{JsonSchema, generate::SchemaSettings};

use crate::{JsonFunctionInfo, JsonTool};

/// The description of a tool to be used in the chat completion request.
pub struct Tool<P: JsonSchema> {
    name: String,
    description: String,
    _p: PhantomData<P>,
}

impl<P: JsonSchema> Tool<P> {
    /// Creates a new tool with the given name, description, and parameters.
    pub fn new(name: String, description: String) -> Self {
        Tool {
            name,
            description,
            _p: PhantomData,
        }
    }

    /// Returns the name of the tool.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the description of the tool.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Converts the tool into a JSON representation.
    pub fn into_json(self) -> JsonTool {
        let parameters = create_parameters_schema::<P>();

        JsonTool {
            tool_type: "function".to_string(),
            function: JsonFunctionInfo {
                name: self.name,
                description: self.description,
                parameters,
                strict: true,
            },
        }
    }
}

/// Creates a JSON schema for the given type `P`.
pub fn create_parameters_schema<P: JsonSchema>() -> Schema {
    let settings = SchemaSettings::default().with_transform(AddNullable::default());
    let generator = settings.into_generator();
    generator.into_root_schema_for::<P>()
}
