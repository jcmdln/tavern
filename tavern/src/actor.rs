use crate::{
    core::Object,
    property,
    traits::{ObjectTrait, StreamTrait},
};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Actor {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Actor {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Actor".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Actor {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Application {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Application {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Application".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Application {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Group {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Group {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Group".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Group {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Organization {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Organization {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Organization".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Organization {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Person {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Person {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Person".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Person {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Service {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Service {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Service".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Service {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}
