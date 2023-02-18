use crate::{core::Object, impl_Object_for, property, traits::StreamTrait};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Article {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Article);

impl Article {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Article".to_string()))
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Audio {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Audio);

impl Audio {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Audio".to_string()))
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Document {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Document);

impl Document {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Document".to_string()))
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Event {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Event);

impl Event {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Event".to_string()))
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Image {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Image);

impl Image {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Image".to_string()))
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Mention {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Mention);

impl Mention {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Mention".to_string()))
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Note {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Note);

impl Note {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Note".to_string()))
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Page {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Page);

impl Page {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Page".to_string()))
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Place {
    pub accuracy: Option<f32>,
    pub altitude: Option<f32>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub radius: Option<f32>,
    pub units: Option<String>,

    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Place);

impl Place {
    pub fn new() -> Place {
        Place::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Place".to_string()))
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub describes: Option<property::Describes>,

    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Profile);

impl Profile {
    pub fn new() -> Profile {
        Profile::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Profile".to_string()))
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Relationship {
    pub object: Option<property::Object>,
    pub relationship: Option<property::Relationship>,
    pub subject: Option<property::Subject>,

    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Relationship);

impl Relationship {
    pub fn new() -> Relationship {
        Relationship::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Relationship".to_string()))
    }
}

#[allow(dead_code, non_snake_case)]
#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Tombstone {
    pub deleted: Option<String>,
    pub formerType: Option<property::FormerType>,

    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Tombstone);

impl Tombstone {
    pub fn new() -> Tombstone {
        Tombstone::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Tombstone".to_string()))
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Video {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Video);

impl Video {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Video".to_string()))
    }
}
