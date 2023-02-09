use crate::{core::Object, traits::StreamTrait};

use serde_json::json;

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Actor(Object);

impl Actor {
    pub fn new() -> Object {
        Object::new().r#type(json!("Actor"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Application(Object);

impl Application {
    pub fn new() -> Object {
        Object::new().r#type(json!("Application"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Group(Object);

impl Group {
    pub fn new() -> Object {
        Object::new().r#type(json!("Group"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Organization(Object);

impl Organization {
    pub fn new() -> Object {
        Object::new().r#type(json!("Organization"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Person(Object);

impl Person {
    pub fn new() -> Object {
        Object::new().r#type(json!("Person"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Service(Object);

impl Service {
    pub fn new() -> Object {
        Object::new().r#type(json!("Service"))
    }
}
