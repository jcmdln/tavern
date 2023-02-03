use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Link {
    #[builder(default = "String::from(\"https://www.w3.org/ns/activitystreams\")")]
    #[serde(rename = "@context")]
    pub context: String,
    #[builder(default = "String::from(\"Link\")")]
    pub r#type: String,

    pub href: String,

    pub hreflang: Option<String>,
    pub media_type: Option<String>,
    pub name: Option<String>,
    pub rel: Option<String>,
    pub preview: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

impl Link {
    pub fn builder(href: &str) -> Link {
        LinkBuilder::default().href(String::from(href)).build().unwrap()
    }
}

#[skip_serializing_none]
#[derive(Builder, Debug, Default, Clone, Deserialize, Serialize)]
#[builder(default)]
pub struct Mention {
    #[builder(default = "String::from(\"https://www.w3.org/ns/activitystreams\")")]
    #[serde(rename = "@context")]
    pub context: String,
    #[builder(default = "String::from(\"Mention\")")]
    pub r#type: String,

    pub href: String,

    pub hreflang: Option<String>,
    pub media_type: Option<String>,
    pub name: Option<String>,
    pub rel: Option<String>,
    pub preview: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

impl Mention {
    pub fn builder(href: &str) -> Mention {
        MentionBuilder::default().href(String::from(href)).build().unwrap()
    }
}
