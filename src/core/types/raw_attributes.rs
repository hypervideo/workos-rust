use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The raw attributes returned from the Identity or Directory Provider.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawAttributes(pub HashMap<String, Value>);

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use serde_json::{Value, json};

    use super::RawAttributes;

    #[test]
    fn it_deserializes_raw_attributes() {
        let raw_attributes: RawAttributes = serde_json::from_str(
            &json!({
                "id": "02grqrue4294w24",
                "name": "Developers",
                "email": "developers@example.com",
                "description": "Software Developers"
            })
            .to_string(),
        )
        .unwrap();

        let mut expected_raw_attributes = HashMap::new();
        expected_raw_attributes.insert(
            "id".to_string(),
            Value::String("02grqrue4294w24".to_string()),
        );
        expected_raw_attributes.insert("name".to_string(), Value::String("Developers".to_string()));
        expected_raw_attributes.insert(
            "email".to_string(),
            Value::String("developers@example.com".to_string()),
        );
        expected_raw_attributes.insert(
            "description".to_string(),
            Value::String("Software Developers".to_string()),
        );

        assert_eq!(raw_attributes, RawAttributes(expected_raw_attributes),)
    }

    #[test]
    fn it_deserializes_raw_attributes_of_different_types() {
        let raw_attributes: RawAttributes = serde_json::from_str(
            &json!({
                "null": null,
                "bool": false,
                "number": 123_i32,
                "string": "A String",
                "array": ["Hello", "world"],
                "object": {
                    "hello": "world"
                }
            })
            .to_string(),
        )
        .unwrap();

        let mut expected_raw_attributes = HashMap::new();
        expected_raw_attributes.insert("null".to_string(), Value::Null);
        expected_raw_attributes.insert("bool".to_string(), Value::Bool(false));
        expected_raw_attributes.insert("number".to_string(), json!(123_i32));
        expected_raw_attributes.insert("string".to_string(), Value::String("A String".to_string()));
        expected_raw_attributes.insert(
            "array".to_string(),
            Value::Array(vec![
                Value::String("Hello".to_string()),
                Value::String("world".to_string()),
            ]),
        );
        expected_raw_attributes.insert("object".to_string(), json!({ "hello": "world" }));

        assert_eq!(raw_attributes, RawAttributes(expected_raw_attributes))
    }
}
