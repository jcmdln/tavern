use crate::{
    core::{Link, Object},
    traits::StreamTrait,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Article(Object);

impl Article {
    pub fn new() -> Object {
        Object::new().r#type(json!("Article"))
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Audio(Object);

impl Audio {
    pub fn new() -> Object {
        Object::new().r#type(json!("Audio"))
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Document(Object);

impl Document {
    pub fn new() -> Object {
        Object::new().r#type(json!("Document"))
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Event(Object);

impl Event {
    pub fn new() -> Object {
        Object::new().r#type(json!("Event"))
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Image(Object);

impl Image {
    pub fn new() -> Object {
        Object::new().r#type(json!("Image"))
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Mention(Link);

impl Mention {
    pub fn new() -> Link {
        Link::default().r#type(json!("Mention"))
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Note(Object);

impl Note {
    pub fn new() -> Object {
        Object::new().r#type(json!("Note"))
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Page(Object);

impl Page {
    pub fn new() -> Object {
        Object::new().r#type(json!("Page"))
    }
}

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
            .at_context(json!("https://www.w3.org/ns/activitystreams"))
            .r#type(json!("Place"))
    }
}

impl StreamTrait for Place {
    fn at_context(&mut self, value: Value) -> Self {
        self.base.at_context = value;
        self.to_owned()
    }
    fn r#type(&mut self, value: Value) -> Self {
        self.base.r#type = value;
        self.to_owned()
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub describes: Option<Value>,

    #[serde(flatten)]
    pub base: Object,
}

impl Profile {
    pub fn new() -> Profile {
        Profile::default()
            .at_context(json!("https://www.w3.org/ns/activitystreams"))
            .r#type(json!("Profile"))
    }
}

impl StreamTrait for Profile {
    fn at_context(&mut self, value: Value) -> Self {
        self.base.at_context = value;
        self.to_owned()
    }
    fn r#type(&mut self, value: Value) -> Self {
        self.base.r#type = value;
        self.to_owned()
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Relationship {
    pub object: Option<Value>,
    pub relationship: Option<Value>,
    pub subject: Option<Value>,

    #[serde(flatten)]
    pub base: Object,
}

impl Relationship {
    pub fn new() -> Relationship {
        Relationship::default()
            .at_context(json!("https://www.w3.org/ns/activitystreams"))
            .r#type(json!("Relationship"))
    }
}

impl StreamTrait for Relationship {
    fn at_context(&mut self, value: Value) -> Self {
        self.base.at_context = value;
        self.to_owned()
    }
    fn r#type(&mut self, value: Value) -> Self {
        self.base.r#type = value;
        self.to_owned()
    }
}

#[allow(dead_code, non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Tombstone {
    pub deleted: Option<String>,
    pub formerType: Option<Value>,

    #[serde(flatten)]
    pub base: Object,
}

impl Tombstone {
    pub fn new() -> Tombstone {
        Tombstone::default()
            .at_context(json!("https://www.w3.org/ns/activitystreams"))
            .r#type(json!("Tombstone"))
    }
}

impl StreamTrait for Tombstone {
    fn at_context(&mut self, value: Value) -> Self {
        self.base.at_context = value;
        self.to_owned()
    }
    fn r#type(&mut self, value: Value) -> Self {
        self.base.r#type = value;
        self.to_owned()
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Video(Object);

impl Video {
    pub fn new() -> Object {
        Object::new().r#type(json!("Video"))
    }
}
