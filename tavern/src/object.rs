use crate::{
    core::Object,
    property,
    traits::{ObjectTrait, StreamTrait},
};
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Article {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Audio {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Document {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mention {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Note {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Page {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Place {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Profile {
    #[serde(flatten)]
    pub extends: Object,

    pub describes: Option<property::Describes>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            extends: Object::default().kind(property::Type::String("Profile".into())).clone(),
            describes: None,
        }
    }
}

impl StreamTrait for Profile {
    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Profile {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Relationship {
    #[serde(flatten)]
    pub extends: Object,

    pub object: Option<property::Object>,
    pub relationship: Option<property::Relationship>,
    pub subject: Option<property::Subject>,
}

impl Default for Relationship {
    fn default() -> Self {
        Self {
            extends: Object::default().kind(property::Type::String("Relationship".into())).clone(),
            object: None,
            relationship: None,
            subject: None,
        }
    }
}

impl StreamTrait for Relationship {
    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Relationship {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tombstone {
    #[serde(flatten)]
    pub extends: Object,

    pub deleted: Option<String>,
    pub former_type: Option<property::FormerType>,
}

impl Default for Tombstone {
    fn default() -> Self {
        Self {
            extends: Object::default().kind(property::Type::String("Tombstone".into())).clone(),
            deleted: None,
            former_type: None,
        }
    }
}

impl StreamTrait for Tombstone {
    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Tombstone {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Video {
    #[serde(flatten)]
    pub extends: Object,
}

macro_rules! object_impl {
    ($($t:ty)*) => ($(
        impl Default for $t {
            fn default() -> Self {
                Self {
                    extends: $crate::traits::StreamTrait::kind(
                        &mut $crate::core::Object::default(),
                        $crate::property::Type::String(stringify!($t).into()),
                    )
                    .clone(),
                }
            }
        }

        impl $crate::traits::StreamTrait for $t {
            fn as_stream(&mut self) -> &mut $crate::core::Stream {
                &mut self.as_object().extends
            }
        }

        impl $crate::traits::ObjectTrait for $t {
            fn as_object(&mut self) -> &mut $crate::core::Object {
                &mut self.extends
            }
        }
    )*)
}

object_impl! { Article Audio Document Event Image Mention Note Page Place Video }
