use oasgen::{OaSchema, ReferenceOr, Schema};

use crate::defined::Defined;

impl<T> OaSchema for Defined<T> where T: OaSchema {
    fn schema_ref() -> ReferenceOr<Schema> {
        let mut schema = T::schema_ref();
        let Some(s) = schema.as_mut() else {
            return schema;
        };
        s.nullable = true;
        schema
    }

    fn schema() -> Schema {
        let mut schema = T::schema();
        schema.nullable = true;
        schema
    }
}