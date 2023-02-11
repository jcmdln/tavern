use crate::{core::Object, property, traits::StreamTrait};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Actor {
    #[serde(flatten)]
    pub base: Object,
}

impl Actor {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Actor".to_string()))
    }
}

impl StreamTrait for Actor {
    fn atContext(&mut self, value: property::AtContext) -> Self {
        self.base.atContext = Some(value);
        self.to_owned()
    }
    fn id(&mut self, value: String) -> Self {
        self.base.id = Some(value);
        self.to_owned()
    }
    fn r#type(&mut self, value: property::Type) -> Self {
        self.base.r#type = Some(value);
        self.to_owned()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Application {
    #[serde(flatten)]
    pub base: Object,
}

impl Application {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Application".to_string()))
    }
}

impl StreamTrait for Application {
    fn atContext(&mut self, value: property::AtContext) -> Self {
        self.base.atContext = Some(value);
        self.to_owned()
    }
    fn id(&mut self, value: String) -> Self {
        self.base.id = Some(value);
        self.to_owned()
    }
    fn r#type(&mut self, value: property::Type) -> Self {
        self.base.r#type = Some(value);
        self.to_owned()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Group {
    #[serde(flatten)]
    pub base: Object,
}

impl Group {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Group".to_string()))
    }
}

impl StreamTrait for Group {
    fn atContext(&mut self, value: property::AtContext) -> Self {
        self.base.atContext = Some(value);
        self.to_owned()
    }
    fn id(&mut self, value: String) -> Self {
        self.base.id = Some(value);
        self.to_owned()
    }
    fn r#type(&mut self, value: property::Type) -> Self {
        self.base.r#type = Some(value);
        self.to_owned()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Organization {
    #[serde(flatten)]
    pub base: Object,
}

impl Organization {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Organization".to_string()))
    }
}

impl StreamTrait for Organization {
    fn atContext(&mut self, value: property::AtContext) -> Self {
        self.base.atContext = Some(value);
        self.to_owned()
    }
    fn id(&mut self, value: String) -> Self {
        self.base.id = Some(value);
        self.to_owned()
    }
    fn r#type(&mut self, value: property::Type) -> Self {
        self.base.r#type = Some(value);
        self.to_owned()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Person {
    #[serde(flatten)]
    pub base: Object,
}

impl Person {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Person".to_string()))
    }
}

impl StreamTrait for Person {
    fn atContext(&mut self, value: property::AtContext) -> Self {
        self.base.atContext = Some(value);
        self.to_owned()
    }
    fn id(&mut self, value: String) -> Self {
        self.base.id = Some(value);
        self.to_owned()
    }
    fn r#type(&mut self, value: property::Type) -> Self {
        self.base.r#type = Some(value);
        self.to_owned()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Service {
    #[serde(flatten)]
    pub base: Object,
}

impl Service {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Service".to_string()))
    }
}

impl StreamTrait for Service {
    fn atContext(&mut self, value: property::AtContext) -> Self {
        self.base.atContext = Some(value);
        self.to_owned()
    }
    fn id(&mut self, value: String) -> Self {
        self.base.id = Some(value);
        self.to_owned()
    }
    fn r#type(&mut self, value: property::Type) -> Self {
        self.base.r#type = Some(value);
        self.to_owned()
    }
}
