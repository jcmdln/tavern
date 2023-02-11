use crate::{core::Object, property, traits::StreamTrait};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Article {
    #[serde(flatten)]
    pub base: Object,
}

impl Article {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Article".to_string()))
    }
}

impl StreamTrait for Article {
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

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Audio {
    #[serde(flatten)]
    pub base: Object,
}

impl Audio {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Audio".to_string()))
    }
}

impl StreamTrait for Audio {
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

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Document {
    #[serde(flatten)]
    pub base: Object,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Document".to_string()))
    }
}

impl StreamTrait for Document {
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

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Event {
    #[serde(flatten)]
    pub base: Object,
}

impl Event {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Event".to_string()))
    }
}

impl StreamTrait for Event {
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

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Image {
    #[serde(flatten)]
    pub base: Object,
}

impl Image {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Image".to_string()))
    }
}

impl StreamTrait for Image {
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

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Mention {
    #[serde(flatten)]
    pub base: Object,
}

impl Mention {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Mention".to_string()))
    }
}

impl StreamTrait for Mention {
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

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Note {
    #[serde(flatten)]
    pub base: Object,
}

impl Note {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Note".to_string()))
    }
}

impl StreamTrait for Note {
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

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Page {
    #[serde(flatten)]
    pub base: Object,
}

impl Page {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Page".to_string()))
    }
}

impl StreamTrait for Page {
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

impl Place {
    pub fn new() -> Place {
        Place::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Place".to_string()))
    }
}

impl StreamTrait for Place {
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

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub describes: Option<property::Describes>,

    #[serde(flatten)]
    pub base: Object,
}

impl Profile {
    pub fn new() -> Profile {
        Profile::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Profile".to_string()))
    }
}

impl StreamTrait for Profile {
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

impl Relationship {
    pub fn new() -> Relationship {
        Relationship::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Relationship".to_string()))
    }
}

impl StreamTrait for Relationship {
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

impl Tombstone {
    pub fn new() -> Tombstone {
        Tombstone::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Tombstone".to_string()))
    }
}

impl StreamTrait for Tombstone {
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

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Video {
    #[serde(flatten)]
    pub base: Object,
}

impl Video {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Video".to_string()))
    }
}

impl StreamTrait for Video {
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
