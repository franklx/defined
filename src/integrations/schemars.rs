use std::borrow::Cow;

use schemars::{JsonSchema, gen::SchemaGenerator, schema::{Schema, SchemaObject, SubschemaValidation, InstanceType, SingleOrVec}};

use crate::defined::Defined;

impl<T: JsonSchema> JsonSchema for Defined<T> {

    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        format!("Omittable_{}", T::schema_name())
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Owned(format!("Defined<{}>", T::schema_id()))
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let mut schema = gen.subschema_for::<T>();
        if gen.settings().option_add_null_type {
            schema = match schema {
                Schema::Bool(true) => Schema::Bool(true),
                Schema::Bool(false) => <()>::json_schema(gen),
                Schema::Object(SchemaObject {
                    instance_type: Some(ref mut instance_type),
                    ..
                }) => {
                    add_null_type(instance_type);
                    schema
                }
                schema => SchemaObject {
                    // TODO technically the schema already accepts null, so this may be unnecessary
                    subschemas: Some(Box::new(SubschemaValidation {
                        any_of: Some(vec![schema, <()>::json_schema(gen)]),
                        ..Default::default()
                    })),
                    ..Default::default()
                }
                .into(),
            }
        }
        if gen.settings().option_nullable {
            let mut schema_obj = schema.into_object();
            schema_obj
                .extensions
                .insert("nullable".to_owned(), serde_json::json!(true));
            schema = Schema::Object(schema_obj);
        };
        schema
    }

    fn _schemars_private_non_optional_json_schema(gen: &mut SchemaGenerator) -> Schema {
        T::_schemars_private_non_optional_json_schema(gen)
    }

    fn _schemars_private_is_option() -> bool {
        true
    }
}

fn add_null_type(instance_type: &mut SingleOrVec<InstanceType>) {
    match instance_type {
        SingleOrVec::Single(ty) if **ty != InstanceType::Null => {
            *instance_type = vec![**ty, InstanceType::Null].into()
        }
        SingleOrVec::Vec(ty) if !ty.contains(&InstanceType::Null) => ty.push(InstanceType::Null),
        _ => {}
    };
}
