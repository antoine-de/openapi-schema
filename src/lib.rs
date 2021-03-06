#[allow(unused_imports)]
#[macro_use]
extern crate openapi_schema_derive;
pub use openapi_schema_derive::*;

use openapi::v3_0::{ObjectOrReference, Schema, Spec};
use serde_json::{Number, Value};

pub trait OpenapiSchema {
    fn generate_schema(spec: &mut Spec) -> ObjectOrReference<Schema>;
}

impl OpenapiSchema for String {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("string".into()),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for i64 {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("number".into()),
            format: Some("int64".into()),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for u64 {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("number".into()),
            format: Some("int64".into()),
            minimum: Some(Value::Number(Number::from(0))),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for i32 {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("number".into()),
            format: Some("int32".into()),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for u32 {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("number".into()),
            format: Some("int32".into()),
            minimum: Some(Value::Number(Number::from(0))),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for bool {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("boolean".into()),
            ..Default::default()
        })
    }
}

impl<T> OpenapiSchema for Option<T>
where
    T: OpenapiSchema,
{
    fn generate_schema(spec: &mut Spec) -> ObjectOrReference<Schema> {
        T::generate_schema(spec)
    }
}

impl<T> OpenapiSchema for Vec<T>
where
    T: OpenapiSchema,
{
    fn generate_schema(spec: &mut Spec) -> ObjectOrReference<Schema> {
        let reference = T::generate_schema(spec);
        let items_schema = match reference {
            ObjectOrReference::Object(schema) => schema,
            ObjectOrReference::Ref { ref_path } => Schema {
                ref_path: Some(ref_path),
                ..Schema::default()
            },
        };

        ObjectOrReference::Object(Schema {
            schema_type: Some("array".into()),
            items: Some(Box::new(items_schema)),
            ..Schema::default()
        })
    }
}
