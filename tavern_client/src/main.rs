use serde_json::{json, Value};
use std::process::exit;

fn main() {
    let value = json!({
        "@context": "https://www.w3.org/ns/activitystreams",
        "type": "Object",
    });

    let value_context = value.get("@context").unwrap();
    match value_context {
        Value::String(v) => Some(Value::String(v.to_owned())),
        Value::Array(v) => Some(Value::Array(v.to_owned())),
        Value::Object(v) => Some(Value::Object(v.to_owned())),
        _ => {
            println!("error: unknown @context");
            exit(1);
        }
    };

    let value_type = value.get("type").unwrap();
    match value_type {
        Value::String(v) => Some(Value::String(v.to_owned())),
        Value::Array(v) => Some(Value::Array(v.to_owned())),
        Value::Object(v) => Some(Value::Object(v.to_owned())),
        _ => {
            println!("error: unknown type");
            exit(1);
        }
    };

    println!("value: {}", value);

    let object = tavern::core::Object::new();
    println!("{}", serde_json::to_string(&object).unwrap());

    let page = tavern::object::Page::new();
    println!("{}", serde_json::to_string(&page).unwrap());

    let place = tavern::object::Place::new();
    println!("{}", serde_json::to_string(&place).unwrap());
}
