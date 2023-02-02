use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LinkOrImage {
    Link(crate::link::Link),
    Image(Image),
    Vec(Box<LinkOrImage>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LinkOrObject {
    Object(Object),
    Link(crate::link::Link),
    String(String),
    Vec(Box<LinkOrObject>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LinkOrString {
    Link(crate::link::Link),
    String(String),
    Vec(Box<LinkOrString>),
}

#[allow(dead_code, non_snake_case)]
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Object {
    #[builder(default = "String::from(\"https://www.w3.org/ns/activitystreams\")")]
    #[serde(rename = "@context")]
    pub at_context: String,
    #[builder(default = "String::from(\"Object\")")]
    pub r#type: String,

    pub attachment: Option<Box<LinkOrObject>>,
    pub attributedTo: Option<Box<LinkOrObject>>,
    pub audience: Option<Box<LinkOrObject>>,
    pub bcc: Option<Box<LinkOrObject>>,
    pub bto: Option<Box<LinkOrObject>>,
    pub cc: Option<Box<LinkOrObject>>,
    pub content: Option<String>,
    pub contentMap: Option<Vec<String>>,
    pub context: Option<Vec<LinkOrObject>>,
    pub duration: Option<i32>,
    pub endTime: Option<String>,
    pub generator: Option<Box<LinkOrObject>>,
    pub icon: Option<Box<LinkOrImage>>,
    pub image: Option<String>,
    pub inReplyTo: Option<String>,
    pub location: Option<String>,
    pub mediaType: Option<String>,
    pub name: Option<String>,
    pub nameMap: Option<Vec<String>>,
    pub preview: Option<String>,
    pub published: Option<String>,
    pub replies: Option<String>,
    pub startTime: Option<String>,
    pub summary: Option<String>,
    pub summaryMap: Option<Vec<String>>,
    pub tag: Option<Vec<LinkOrObject>>,
    pub to: Option<Box<LinkOrObject>>,
    pub updated: Option<String>,
    pub url: Option<LinkOrString>,
}

#[allow(dead_code)]
impl Object {
    pub fn builder() -> Object {
        ObjectBuilder::default().build().unwrap()
    }
}

#[allow(dead_code)]
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Article {
    #[serde(flatten)]
    base: Option<Object>,
}

#[allow(dead_code)]
impl Article {
    pub fn builder() -> Article {
        let base = ObjectBuilder::default().r#type("Article".to_string()).build().unwrap();
        ArticleBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[allow(dead_code)]
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Audio {
    #[serde(flatten)]
    base: Option<Object>,
}

#[allow(dead_code)]
impl Audio {
    pub fn builder() -> Audio {
        let base = ObjectBuilder::default().r#type("Audio".to_string()).build().unwrap();
        AudioBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[allow(dead_code)]
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Document {
    #[serde(flatten)]
    base: Option<Object>,
}

#[allow(dead_code)]
impl Document {
    pub fn builder() -> Document {
        let base = ObjectBuilder::default().r#type("Document".to_string()).build().unwrap();
        DocumentBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[allow(dead_code)]
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Event {
    #[serde(flatten)]
    base: Option<Object>,
}

#[allow(dead_code)]
impl Event {
    pub fn builder() -> Event {
        let base = ObjectBuilder::default().r#type("Event".to_string()).build().unwrap();
        EventBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[allow(dead_code)]
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Image {
    #[serde(flatten)]
    base: Option<Object>,
}

#[allow(dead_code)]
impl Image {
    pub fn builder() -> Image {
        let base = ObjectBuilder::default().r#type("Image".to_string()).build().unwrap();
        ImageBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[allow(dead_code)]
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Note {
    #[serde(flatten)]
    base: Option<Object>,
}

#[allow(dead_code)]
impl Note {
    pub fn builder() -> Note {
        let base = ObjectBuilder::default().r#type("Note".to_string()).build().unwrap();
        NoteBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[allow(dead_code)]
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Page {
    #[serde(flatten)]
    base: Option<Object>,
}

#[allow(dead_code)]
impl Page {
    pub fn builder() -> Page {
        let base = ObjectBuilder::default().r#type("Page".to_string()).build().unwrap();
        PageBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
impl Place {
    pub fn builder() -> Place {
        let base = ObjectBuilder::default().r#type("Place".to_string()).build().unwrap();
        PlaceBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[allow(dead_code)]
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Profile {
    describes: Option<Object>,

    #[serde(flatten)]
    base: Option<Object>,
}

#[allow(dead_code)]
impl Profile {
    pub fn builder() -> Profile {
        let base = ObjectBuilder::default().r#type("Profile".to_string()).build().unwrap();
        ProfileBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
impl Relationship {
    pub fn builder() -> Relationship {
        let base = ObjectBuilder::default().r#type("Relationship".to_string()).build().unwrap();
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

#[allow(dead_code)]
impl Tombstone {
    pub fn builder() -> Tombstone {
        let base = ObjectBuilder::default().r#type("Tombstone".to_string()).build().unwrap();
        TombstoneBuilder::default().base(Some(base)).build().unwrap()
    }
}

#[allow(dead_code)]
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Video {
    #[serde(flatten)]
    base: Option<Object>,
}

#[allow(dead_code)]
impl Video {
    pub fn builder() -> Video {
        let base = ObjectBuilder::default().r#type("Video".to_string()).build().unwrap();
        VideoBuilder::default().base(Some(base)).build().unwrap()
    }
}
