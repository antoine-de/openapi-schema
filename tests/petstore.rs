use openapi::v3_0::{ObjectOrReference, Spec};
use openapi_schema::OpenapiSchema;

/// Pet Tag
///
/// A tag for a pet
#[allow(dead_code)]
#[derive(OpenapiSchema)]
struct Tag {
    id: i64,
    name: Option<String>,
}

/// A category for a pet
#[derive(OpenapiSchema)]
#[allow(dead_code)]
struct Category {
    id: Option<i64>,
    name: Option<String>,
}

#[test]
fn test_simple_derive() {
    let mut spec = Spec::default();
    Tag::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    let tag = match schemas.get("Tag") {
        Some(ObjectOrReference::Object(ref tag)) => tag,
        _ => panic!("unexpected reference"),
    };

    assert_eq!(tag.title, Some("Pet Tag".into()));
    assert_eq!(tag.description, Some("A tag for a pet".into()));

    let properties = tag.properties.as_ref().unwrap();

    assert!(properties.contains_key("id"));
    let id = properties.get("id").unwrap();
    assert_eq!(id.schema_type, Some("number".into()));
    assert_eq!(id.format, Some("int64".into()));

    assert!(properties.contains_key("name"));
    let name = properties.get("name").unwrap();
    assert_eq!(name.schema_type, Some("string".into()));

    assert_eq!(tag.required, Some(vec![String::from("id")]));

    Category::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    let cat = match schemas.get("Category") {
        Some(ObjectOrReference::Object(ref cat)) => cat,
        _ => panic!("missing Category in schemas"),
    };

    assert_eq!(cat.title, None);
    assert_eq!(cat.description, Some("A category for a pet".into()));

    let properties = cat.properties.as_ref().unwrap();

    assert!(properties.contains_key("id"));
    let id = properties.get("id").unwrap();
    assert_eq!(id.schema_type, Some("number".into()));
    assert_eq!(id.format, Some("int64".into()));

    assert!(properties.contains_key("name"));
    let name = properties.get("name").unwrap();
    assert_eq!(name.schema_type, Some("string".into()));

    assert_eq!(cat.required, None);
}
