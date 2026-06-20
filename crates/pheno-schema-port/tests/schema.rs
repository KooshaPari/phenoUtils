// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 KooshaPari <kooshapari@gmail.com>

use ports::adapters::json_schema::JsonSchemaAdapter;
use ports::adapters::zod::ZodAdapter;
use ports::schema::{Schema, SchemaFormat};
use serde_json::json;

#[test]
fn json_schema_format() {
    let s = JsonSchemaAdapter::new(json!({}));
    assert_eq!(s.format(), SchemaFormat::JsonSchema);
}

#[test]
fn zod_format() {
    let s = ZodAdapter::new(json!({}));
    assert_eq!(s.format(), SchemaFormat::Zod);
}

#[test]
fn json_schema_validate_ok() {
    let s = JsonSchemaAdapter::new(json!({}));
    let r = s.validate(&json!({}));
    assert!(r.ok);
}

#[test]
fn zod_validate_ok() {
    let s = ZodAdapter::new(json!({}));
    let r = s.validate(&json!("anything"));
    assert!(r.ok);
}

#[test]
fn trait_object_safe() {
    let _t: Box<dyn Schema> = Box::new(JsonSchemaAdapter::new(json!({})));
}
