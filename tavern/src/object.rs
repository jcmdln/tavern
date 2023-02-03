use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

#[allow(dead_code, non_snake_case)]
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Object {
    #[builder(default = "json!(\"https://www.w3.org/ns/activitystreams\")")]
    #[serde(rename = "@context")]
    pub at_context: Value,

    #[builder(default = "json!(\"Object\")")]
    pub r#type: Value,

    pub name: Option<String>,
    pub nameMap: Option<Value>,
    pub summary: Option<String>,
    pub summaryMap: Option<Value>,
    pub content: Option<String>,
    pub contentMap: Option<Value>,

    pub attributedTo: Option<Value>,
    pub inReplyTo: Option<Value>,
    pub published: Option<String>, // FIXME: datetime

    pub to: Option<Value>,
    pub bto: Option<Value>,
    pub cc: Option<Value>,
    pub bcc: Option<Value>,

    pub attachment: Option<Value>,
    pub audience: Option<Value>,
    pub context: Option<Value>,
    pub generator: Option<Value>,
    pub icon: Option<Value>,
    pub replies: Option<Value>,
    pub location: Option<Value>,
    pub preview: Option<Value>,
    pub tag: Option<Value>,
    pub updated: Option<Value>,
    pub url: Option<Value>,

    pub mediaType: Option<Value>,
    pub image: Option<Value>,
    pub duration: Option<f32>,
    pub startTime: Option<String>, // FIXME: datetime
    pub endTime: Option<String>,   // FIXME: datetime
}

impl Object {
    pub fn builder() -> Object {
        ObjectBuilder::default().build().unwrap()
    }
}

#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Article {
    #[serde(flatten)]
    base: Option<Object>,
}

impl Article {
    pub fn builder() -> Article {
        let base = ObjectBuilder::default().r#type(json!("Article")).build().unwrap();
        ArticleBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Audio {
    #[serde(flatten)]
    base: Option<Object>,
}

impl Audio {
    pub fn builder() -> Audio {
        let base = ObjectBuilder::default().r#type(json!("Audio")).build().unwrap();
        AudioBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Document {
    #[serde(flatten)]
    base: Option<Object>,
}

impl Document {
    pub fn builder() -> Document {
        let base = ObjectBuilder::default().r#type(json!("Document")).build().unwrap();
        DocumentBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Event {
    #[serde(flatten)]
    base: Option<Object>,
}

impl Event {
    pub fn builder() -> Event {
        let base = ObjectBuilder::default().r#type(json!("Event")).build().unwrap();
        EventBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Image {
    #[serde(flatten)]
    base: Option<Object>,
}

impl Image {
    pub fn builder() -> Image {
        let base = ObjectBuilder::default().r#type(json!("Image")).build().unwrap();
        ImageBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Note {
    #[serde(flatten)]
    base: Option<Object>,
}

impl Note {
    pub fn builder() -> Note {
        let base = ObjectBuilder::default().r#type(json!("Note")).build().unwrap();
        NoteBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Page {
    #[serde(flatten)]
    base: Option<Object>,
}

impl Page {
    pub fn builder() -> Page {
        let base = ObjectBuilder::default().r#type(json!("Page")).build().unwrap();
        PageBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Place {
    accuracy: Option<f32>,
    altitude: Option<f32>,
    latitude: Option<f32>,
    longitude: Option<f32>,
    radius: Option<f32>,
    units: Option<String>,

    #[serde(flatten)]
    base: Option<Object>,
}

impl Place {
    pub fn builder() -> Place {
        let base = ObjectBuilder::default().r#type(json!("Place")).build().unwrap();
        PlaceBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Profile {
    describes: Option<Object>,

    #[serde(flatten)]
    base: Option<Object>,
}

impl Profile {
    pub fn builder() -> Profile {
        let base = ObjectBuilder::default().r#type(json!("Profile")).build().unwrap();
        ProfileBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Relationship {
    object: Option<Object>,
    relationship: Option<String>,
    subject: Option<String>,

    #[serde(flatten)]
    base: Option<Object>,
}

impl Relationship {
    pub fn builder() -> Relationship {
        let base = ObjectBuilder::default().r#type(json!("Relationship")).build().unwrap();
        RelationshipBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[allow(dead_code, non_snake_case)]
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Tombstone {
    formerType: Option<String>,
    deleted: Option<String>,

    #[serde(flatten)]
    base: Option<Object>,
}

impl Tombstone {
    pub fn builder() -> Tombstone {
        let base = ObjectBuilder::default().r#type(json!("Tombstone")).build().unwrap();
        TombstoneBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Video {
    #[serde(flatten)]
    base: Option<Object>,
}

impl Video {
    pub fn builder() -> Video {
        let base = ObjectBuilder::default().r#type(json!("Video")).build().unwrap();
        VideoBuilder::default().base(Some(base)).build().unwrap()
    }
}
