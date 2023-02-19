use crate::{
    core::Object,
    property,
    traits::{ObjectTrait, StreamTrait},
};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Article {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Article {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Article".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Article {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Audio {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Audio {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Audio".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Audio {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Document {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Document {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Document".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Document {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Event {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Event {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Event".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Event {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Image {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Image {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Image".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Image {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Mention {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Mention {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Mention".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Mention {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Note {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Note {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Note".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Note {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Page {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Page {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Page".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Page {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Place {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Place {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Place".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Place {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Profile {
    #[serde(flatten)]
    pub extends: Object,

    pub describes: Option<property::Describes>,
}

impl StreamTrait for Profile {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Profile".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Profile {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Relationship {
    #[serde(flatten)]
    pub extends: Object,

    pub object: Option<property::Object>,
    pub relationship: Option<property::Relationship>,
    pub subject: Option<property::Subject>,
}

impl StreamTrait for Relationship {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Relationship".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Relationship {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[allow(dead_code, non_snake_case)]
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Tombstone {
    #[serde(flatten)]
    pub extends: Object,

    pub deleted: Option<String>,
    pub formerType: Option<property::FormerType>,
}

impl StreamTrait for Tombstone {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Tombstone".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Tombstone {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Video {
    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for Video {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Video".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Video {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}
