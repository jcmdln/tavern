use crate::{core::Object, impl_Object_for, property, traits::StreamTrait};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Actor {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Actor);

impl Actor {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Actor".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Application {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Application);

impl Application {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Application".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Group {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Group);

impl Group {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Group".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Organization {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Organization);

impl Organization {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Organization".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Person {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Person);

impl Person {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Person".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Service {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Service);

impl Service {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Service".to_string()))
    }
}
