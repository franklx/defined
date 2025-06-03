use std::borrow::Cow;

use schemars::{json_schema, JsonSchema, Schema, SchemaGenerator};
use serde_json::{Map, Value};

use crate::defined::Defined;

fn contains_immediate_subschema(schema_obj: &Map<String, Value>) -> bool {
    ["if", "allOf", "anyOf", "oneOf", "$ref"]
        .into_iter()
        .any(|k| schema_obj.contains_key(k))
}

pub(crate) fn allow_null(generator: &mut SchemaGenerator, schema: &mut Schema) {

    match schema.as_object_mut() {
        Some(obj) => {
            if obj.len() == 1
                && obj
                    .get("anyOf")
                    .and_then(Value::as_array)
                    .is_some_and(|a| a.iter().any(Value::is_null))
            {
                return;
            }

            if contains_immediate_subschema(obj) {
                *schema = json_schema!({
                    "anyOf": [
                        obj,
                        <()>::json_schema(generator)
                    ]
                });
                // No need to check `type`/`const`/`enum` because they're trivially not present
                return;
            }

            if let Some(instance_type) = obj.get_mut("type") {
                match instance_type {
                    Value::Array(array) => {
                        let null = Value::from("null");
                        if !array.contains(&null) {
                            array.push(null);
                        }
                    }
                    Value::String(string) => {
                        if string != "null" {
                            let current_type = core::mem::take(string).into();
                            *instance_type = Value::Array(vec![current_type, "null".into()]);
                        }
                    }
                    _ => {}
                }
            }

            if let Some(c) = obj.remove("const") {
                if !c.is_null() {
                    obj.insert("enum".to_string(), Value::Array(vec![c, Value::Null]));
                }
            } else if let Some(Value::Array(e)) = obj.get_mut("enum") {
                if !e.contains(&Value::Null) {
                    e.push(Value::Null);
                }
            }
        }
        None => {
            *schema = <()>::json_schema(generator);
        }
    }
}

impl<T: JsonSchema> JsonSchema for Defined<T> {

    fn schema_name() -> Cow<'static, str> {
        format!("Omittable_{}", T::schema_name()).into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!("Defined<{}>", T::schema_id()).into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let mut schema = generator.subschema_for::<T>();
        allow_null(generator, &mut schema);
        schema
    }

    fn _schemars_private_non_optional_json_schema(generator: &mut SchemaGenerator) -> Schema {
        T::_schemars_private_non_optional_json_schema(generator)
    }

    fn _schemars_private_is_option() -> bool {
        true
    }
}