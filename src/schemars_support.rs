use std::borrow::Cow;

use schemars::{json_schema, JsonSchema};

use crate::NonEmptyString;

impl JsonSchema for NonEmptyString {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "nonemtpy_string".into()
    }
    fn inline_schema() -> bool {
        true
    }
    fn schema_id() -> Cow<'static, str> {
        "nonempty_string".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "minLength": 1,
            "title": "Non-Empty String",
            "description": "A string that must contain at least one character"
        })
    }
}
