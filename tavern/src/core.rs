use crate::{
    property,
    traits::{LinkTrait, ObjectTrait, StreamTrait},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Object {
    // Stream
    #[serde(rename = "@context")]
    pub atContext: Option<property::AtContext>,
    pub id: Option<String>,
    pub r#type: Option<property::Type>,
    // Object
    pub attachment: Option<property::Attachment>,
    pub attributedTo: Option<property::AttributedTo>,
    pub audience: Option<property::Audience>,
    pub bcc: Option<property::Bcc>,
    pub bto: Option<property::Bto>,
    pub cc: Option<property::Cc>,
    pub content: Option<String>,
    pub contentMap: Option<HashMap<String, String>>,
    pub context: Option<property::Context>,
    pub duration: Option<String>,
    pub endTime: Option<String>,
    pub generator: Option<property::Generator>,
    pub icon: Option<property::Icon>,
    pub image: Option<property::Image>,
    pub inReplyTo: Option<property::InReplyTo>,
    pub location: Option<property::Location>,
    pub mediaType: Option<String>,
    pub name: Option<String>,
    pub nameMap: Option<HashMap<String, String>>,
    pub preview: Option<property::Preview>,
    pub published: Option<String>,
    pub replies: Option<property::Replies>,
    pub startTime: Option<String>,
    pub summary: Option<String>,
    pub summaryMap: Option<HashMap<String, String>>,
    pub tag: Option<property::Tag>,
    pub to: Option<property::To>,
    pub updated: Option<String>,
    pub url: Option<property::Url>,
}

impl Object {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Object".to_string()))
    }
}

impl StreamTrait for Object {
    fn atContext(&mut self, value: property::AtContext) -> Self {
        self.atContext = Some(value);
        self.to_owned()
    }
    fn id(&mut self, value: String) -> Self {
        self.id = Some(value);
        self.to_owned()
    }
    fn r#type(&mut self, value: property::Type) -> Self {
        self.r#type = Some(value);
        self.to_owned()
    }
}

impl ObjectTrait for Object {
    fn attachment(&mut self, value: property::Attachment) -> Self {
        self.attachment = Some(value);
        self.to_owned()
    }
    fn attributedTo(&mut self, value: property::AttributedTo) -> Self {
        self.attributedTo = Some(value);
        self.to_owned()
    }
    fn audience(&mut self, value: property::Audience) -> Self {
        self.audience = Some(value);
        self.to_owned()
    }
    fn bcc(&mut self, value: property::Bcc) -> Self {
        self.bcc = Some(value);
        self.to_owned()
    }
    fn bto(&mut self, value: property::Bto) -> Self {
        self.bto = Some(value);
        self.to_owned()
    }
    fn cc(&mut self, value: property::Cc) -> Self {
        self.cc = Some(value);
        self.to_owned()
    }
    fn content(&mut self, value: String) -> Self {
        self.content = Some(value);
        self.to_owned()
    }
    fn contentMap(&mut self, value: HashMap<String, String>) -> Self {
        self.contentMap = Some(value);
        self.to_owned()
    }
    fn context(&mut self, value: property::Context) -> Self {
        self.context = Some(value);
        self.to_owned()
    }
    fn duration(&mut self, value: String) -> Self {
        self.duration = Some(value);
        self.to_owned()
    }
    fn endTime(&mut self, value: String) -> Self {
        self.endTime = Some(value);
        self.to_owned()
    }
    fn generator(&mut self, value: property::Generator) -> Self {
        self.generator = Some(value);
        self.to_owned()
    }
    fn icon(&mut self, value: property::Icon) -> Self {
        self.icon = Some(value);
        self.to_owned()
    }
    fn image(&mut self, value: property::Image) -> Self {
        self.image = Some(value);
        self.to_owned()
    }
    fn inReplyTo(&mut self, value: property::InReplyTo) -> Self {
        self.inReplyTo = Some(value);
        self.to_owned()
    }
    fn location(&mut self, value: property::Location) -> Self {
        self.location = Some(value);
        self.to_owned()
    }
    fn mediaType(&mut self, value: String) -> Self {
        self.mediaType = Some(value);
        self.to_owned()
    }
    fn name(&mut self, value: String) -> Self {
        self.name = Some(value);
        self.to_owned()
    }
    fn nameMap(&mut self, value: HashMap<String, String>) -> Self {
        self.nameMap = Some(value);
        self.to_owned()
    }
    fn preview(&mut self, value: property::Preview) -> Self {
        self.preview = Some(value);
        self.to_owned()
    }
    fn published(&mut self, value: String) -> Self {
        self.published = Some(value);
        self.to_owned()
    }
    fn replies(&mut self, value: property::Replies) -> Self {
        self.replies = Some(value);
        self.to_owned()
    }
    fn startTime(&mut self, value: String) -> Self {
        self.startTime = Some(value);
        self.to_owned()
    }
    fn summary(&mut self, value: String) -> Self {
        self.summary = Some(value);
        self.to_owned()
    }
    fn summaryMap(&mut self, value: HashMap<String, String>) -> Self {
        self.summaryMap = Some(value);
        self.to_owned()
    }
    fn tag(&mut self, value: property::Tag) -> Self {
        self.tag = Some(value);
        self.to_owned()
    }
    fn to(&mut self, value: property::To) -> Self {
        self.to = Some(value);
        self.to_owned()
    }
    fn updated(&mut self, value: String) -> Self {
        self.updated = Some(value);
        self.to_owned()
    }
    fn url(&mut self, value: property::Url) -> Self {
        self.url = Some(value);
        self.to_owned()
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Link {
    // Stream
    #[serde(rename = "@context")]
    pub atContext: Option<property::AtContext>,
    pub id: Option<String>,
    pub r#type: Option<property::Type>,
    // Link
    pub href: Option<String>,
    pub hreflang: Option<String>,
    pub mediaType: Option<String>,
    pub name: Option<String>,
    pub rel: Option<property::Rel>,
    pub preview: Option<property::Preview>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

impl Link {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Link".to_string()))
    }
}

impl StreamTrait for Link {
    fn atContext(&mut self, value: property::AtContext) -> Self {
        self.atContext = Some(value);
        self.to_owned()
    }
    fn id(&mut self, value: String) -> Self {
        self.id = Some(value);
        self.to_owned()
    }
    fn r#type(&mut self, value: property::Type) -> Self {
        self.r#type = Some(value);
        self.to_owned()
    }
}

impl LinkTrait for Link {
    fn href(&mut self, value: String) -> Self {
        self.href = Some(value);
        self.to_owned()
    }
    fn rel(&mut self, value: property::Rel) -> Self {
        self.rel = Some(value);
        self.to_owned()
    }
    fn mediaType(&mut self, value: String) -> Self {
        self.mediaType = Some(value);
        self.to_owned()
    }
    fn name(&mut self, value: String) -> Self {
        self.name = Some(value);
        self.to_owned()
    }
    fn hreflang(&mut self, value: String) -> Self {
        self.hreflang = Some(value);
        self.to_owned()
    }
    fn height(&mut self, value: u64) -> Self {
        self.height = Some(value);
        self.to_owned()
    }
    fn width(&mut self, value: u64) -> Self {
        self.width = Some(value);
        self.to_owned()
    }
    fn preview(&mut self, value: property::Preview) -> Self {
        self.preview = Some(value);
        self.to_owned()
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Activity {
    pub actor: Option<property::Actor>,
    pub object: Option<property::Object>,
    pub target: Option<property::Target>,
    pub result: Option<property::Result>,
    pub origin: Option<property::Origin>,
    pub instrument: Option<property::Instrument>,

    #[serde(flatten)]
    pub base: Object,
}

impl Activity {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Activity".to_string()))
    }
}

impl StreamTrait for Activity {
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

impl ObjectTrait for Activity {
    fn attachment(&mut self, value: property::Attachment) -> Self {
        self.base.attachment = Some(value);
        self.to_owned()
    }
    fn attributedTo(&mut self, value: property::AttributedTo) -> Self {
        self.base.attributedTo = Some(value);
        self.to_owned()
    }
    fn audience(&mut self, value: property::Audience) -> Self {
        self.base.audience = Some(value);
        self.to_owned()
    }
    fn bcc(&mut self, value: property::Bcc) -> Self {
        self.base.bcc = Some(value);
        self.to_owned()
    }
    fn bto(&mut self, value: property::Bto) -> Self {
        self.base.bto = Some(value);
        self.to_owned()
    }
    fn cc(&mut self, value: property::Cc) -> Self {
        self.base.cc = Some(value);
        self.to_owned()
    }
    fn content(&mut self, value: String) -> Self {
        self.base.content = Some(value);
        self.to_owned()
    }
    fn contentMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.contentMap = Some(value);
        self.to_owned()
    }
    fn context(&mut self, value: property::Context) -> Self {
        self.base.context = Some(value);
        self.to_owned()
    }
    fn duration(&mut self, value: String) -> Self {
        self.base.duration = Some(value);
        self.to_owned()
    }
    fn endTime(&mut self, value: String) -> Self {
        self.base.endTime = Some(value);
        self.to_owned()
    }
    fn generator(&mut self, value: property::Generator) -> Self {
        self.base.generator = Some(value);
        self.to_owned()
    }
    fn icon(&mut self, value: property::Icon) -> Self {
        self.base.icon = Some(value);
        self.to_owned()
    }
    fn image(&mut self, value: property::Image) -> Self {
        self.base.image = Some(value);
        self.to_owned()
    }
    fn inReplyTo(&mut self, value: property::InReplyTo) -> Self {
        self.base.inReplyTo = Some(value);
        self.to_owned()
    }
    fn location(&mut self, value: property::Location) -> Self {
        self.base.location = Some(value);
        self.to_owned()
    }
    fn mediaType(&mut self, value: String) -> Self {
        self.base.mediaType = Some(value);
        self.to_owned()
    }
    fn name(&mut self, value: String) -> Self {
        self.base.name = Some(value);
        self.to_owned()
    }
    fn nameMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.nameMap = Some(value);
        self.to_owned()
    }
    fn preview(&mut self, value: property::Preview) -> Self {
        self.base.preview = Some(value);
        self.to_owned()
    }
    fn published(&mut self, value: String) -> Self {
        self.base.published = Some(value);
        self.to_owned()
    }
    fn replies(&mut self, value: property::Replies) -> Self {
        self.base.replies = Some(value);
        self.to_owned()
    }
    fn startTime(&mut self, value: String) -> Self {
        self.base.startTime = Some(value);
        self.to_owned()
    }
    fn summary(&mut self, value: String) -> Self {
        self.base.summary = Some(value);
        self.to_owned()
    }
    fn summaryMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.summaryMap = Some(value);
        self.to_owned()
    }
    fn tag(&mut self, value: property::Tag) -> Self {
        self.base.tag = Some(value);
        self.to_owned()
    }
    fn to(&mut self, value: property::To) -> Self {
        self.base.to = Some(value);
        self.to_owned()
    }
    fn updated(&mut self, value: String) -> Self {
        self.base.updated = Some(value);
        self.to_owned()
    }
    fn url(&mut self, value: property::Url) -> Self {
        self.base.url = Some(value);
        self.to_owned()
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct IntransitiveActivity {
    pub anyOf: Option<property::AnyOf>,
    pub closed: Option<property::Closed>,
    pub oneOf: Option<property::OneOf>,

    #[serde(flatten)]
    pub base: Object,
}

impl IntransitiveActivity {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("IntransitiveActivity".to_string()))
    }
}

impl StreamTrait for IntransitiveActivity {
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

impl ObjectTrait for IntransitiveActivity {
    fn attachment(&mut self, value: property::Attachment) -> Self {
        self.base.attachment = Some(value);
        self.to_owned()
    }
    fn attributedTo(&mut self, value: property::AttributedTo) -> Self {
        self.base.attributedTo = Some(value);
        self.to_owned()
    }
    fn audience(&mut self, value: property::Audience) -> Self {
        self.base.audience = Some(value);
        self.to_owned()
    }
    fn bcc(&mut self, value: property::Bcc) -> Self {
        self.base.bcc = Some(value);
        self.to_owned()
    }
    fn bto(&mut self, value: property::Bto) -> Self {
        self.base.bto = Some(value);
        self.to_owned()
    }
    fn cc(&mut self, value: property::Cc) -> Self {
        self.base.cc = Some(value);
        self.to_owned()
    }
    fn content(&mut self, value: String) -> Self {
        self.base.content = Some(value);
        self.to_owned()
    }
    fn contentMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.contentMap = Some(value);
        self.to_owned()
    }
    fn context(&mut self, value: property::Context) -> Self {
        self.base.context = Some(value);
        self.to_owned()
    }
    fn duration(&mut self, value: String) -> Self {
        self.base.duration = Some(value);
        self.to_owned()
    }
    fn endTime(&mut self, value: String) -> Self {
        self.base.endTime = Some(value);
        self.to_owned()
    }
    fn generator(&mut self, value: property::Generator) -> Self {
        self.base.generator = Some(value);
        self.to_owned()
    }
    fn icon(&mut self, value: property::Icon) -> Self {
        self.base.icon = Some(value);
        self.to_owned()
    }
    fn image(&mut self, value: property::Image) -> Self {
        self.base.image = Some(value);
        self.to_owned()
    }
    fn inReplyTo(&mut self, value: property::InReplyTo) -> Self {
        self.base.inReplyTo = Some(value);
        self.to_owned()
    }
    fn location(&mut self, value: property::Location) -> Self {
        self.base.location = Some(value);
        self.to_owned()
    }
    fn mediaType(&mut self, value: String) -> Self {
        self.base.mediaType = Some(value);
        self.to_owned()
    }
    fn name(&mut self, value: String) -> Self {
        self.base.name = Some(value);
        self.to_owned()
    }
    fn nameMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.nameMap = Some(value);
        self.to_owned()
    }
    fn preview(&mut self, value: property::Preview) -> Self {
        self.base.preview = Some(value);
        self.to_owned()
    }
    fn published(&mut self, value: String) -> Self {
        self.base.published = Some(value);
        self.to_owned()
    }
    fn replies(&mut self, value: property::Replies) -> Self {
        self.base.replies = Some(value);
        self.to_owned()
    }
    fn startTime(&mut self, value: String) -> Self {
        self.base.startTime = Some(value);
        self.to_owned()
    }
    fn summary(&mut self, value: String) -> Self {
        self.base.summary = Some(value);
        self.to_owned()
    }
    fn summaryMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.summaryMap = Some(value);
        self.to_owned()
    }
    fn tag(&mut self, value: property::Tag) -> Self {
        self.base.tag = Some(value);
        self.to_owned()
    }
    fn to(&mut self, value: property::To) -> Self {
        self.base.to = Some(value);
        self.to_owned()
    }
    fn updated(&mut self, value: String) -> Self {
        self.base.updated = Some(value);
        self.to_owned()
    }
    fn url(&mut self, value: property::Url) -> Self {
        self.base.url = Some(value);
        self.to_owned()
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Collection {
    pub totalItems: Option<u64>,
    pub current: Option<property::Current>,
    pub first: Option<property::First>,
    pub last: Option<property::Last>,
    pub items: Option<property::Items>,

    #[serde(flatten)]
    pub base: Object,
}

impl Collection {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Collection".to_string()))
    }
}

impl StreamTrait for Collection {
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

impl ObjectTrait for Collection {
    fn attachment(&mut self, value: property::Attachment) -> Self {
        self.base.attachment = Some(value);
        self.to_owned()
    }
    fn attributedTo(&mut self, value: property::AttributedTo) -> Self {
        self.base.attributedTo = Some(value);
        self.to_owned()
    }
    fn audience(&mut self, value: property::Audience) -> Self {
        self.base.audience = Some(value);
        self.to_owned()
    }
    fn bcc(&mut self, value: property::Bcc) -> Self {
        self.base.bcc = Some(value);
        self.to_owned()
    }
    fn bto(&mut self, value: property::Bto) -> Self {
        self.base.bto = Some(value);
        self.to_owned()
    }
    fn cc(&mut self, value: property::Cc) -> Self {
        self.base.cc = Some(value);
        self.to_owned()
    }
    fn content(&mut self, value: String) -> Self {
        self.base.content = Some(value);
        self.to_owned()
    }
    fn contentMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.contentMap = Some(value);
        self.to_owned()
    }
    fn context(&mut self, value: property::Context) -> Self {
        self.base.context = Some(value);
        self.to_owned()
    }
    fn duration(&mut self, value: String) -> Self {
        self.base.duration = Some(value);
        self.to_owned()
    }
    fn endTime(&mut self, value: String) -> Self {
        self.base.endTime = Some(value);
        self.to_owned()
    }
    fn generator(&mut self, value: property::Generator) -> Self {
        self.base.generator = Some(value);
        self.to_owned()
    }
    fn icon(&mut self, value: property::Icon) -> Self {
        self.base.icon = Some(value);
        self.to_owned()
    }
    fn image(&mut self, value: property::Image) -> Self {
        self.base.image = Some(value);
        self.to_owned()
    }
    fn inReplyTo(&mut self, value: property::InReplyTo) -> Self {
        self.base.inReplyTo = Some(value);
        self.to_owned()
    }
    fn location(&mut self, value: property::Location) -> Self {
        self.base.location = Some(value);
        self.to_owned()
    }
    fn mediaType(&mut self, value: String) -> Self {
        self.base.mediaType = Some(value);
        self.to_owned()
    }
    fn name(&mut self, value: String) -> Self {
        self.base.name = Some(value);
        self.to_owned()
    }
    fn nameMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.nameMap = Some(value);
        self.to_owned()
    }
    fn preview(&mut self, value: property::Preview) -> Self {
        self.base.preview = Some(value);
        self.to_owned()
    }
    fn published(&mut self, value: String) -> Self {
        self.base.published = Some(value);
        self.to_owned()
    }
    fn replies(&mut self, value: property::Replies) -> Self {
        self.base.replies = Some(value);
        self.to_owned()
    }
    fn startTime(&mut self, value: String) -> Self {
        self.base.startTime = Some(value);
        self.to_owned()
    }
    fn summary(&mut self, value: String) -> Self {
        self.base.summary = Some(value);
        self.to_owned()
    }
    fn summaryMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.summaryMap = Some(value);
        self.to_owned()
    }
    fn tag(&mut self, value: property::Tag) -> Self {
        self.base.tag = Some(value);
        self.to_owned()
    }
    fn to(&mut self, value: property::To) -> Self {
        self.base.to = Some(value);
        self.to_owned()
    }
    fn updated(&mut self, value: String) -> Self {
        self.base.updated = Some(value);
        self.to_owned()
    }
    fn url(&mut self, value: property::Url) -> Self {
        self.base.url = Some(value);
        self.to_owned()
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct OrderedCollection {
    pub totalItems: Option<u64>,
    pub current: Option<property::Current>,
    pub first: Option<property::First>,
    pub last: Option<property::Last>,
    pub items: Option<property::Items>,

    #[serde(flatten)]
    pub base: Object,
}

impl OrderedCollection {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("OrderedCollection".to_string()))
    }
}

impl StreamTrait for OrderedCollection {
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

impl ObjectTrait for OrderedCollection {
    fn attachment(&mut self, value: property::Attachment) -> Self {
        self.base.attachment = Some(value);
        self.to_owned()
    }
    fn attributedTo(&mut self, value: property::AttributedTo) -> Self {
        self.base.attributedTo = Some(value);
        self.to_owned()
    }
    fn audience(&mut self, value: property::Audience) -> Self {
        self.base.audience = Some(value);
        self.to_owned()
    }
    fn bcc(&mut self, value: property::Bcc) -> Self {
        self.base.bcc = Some(value);
        self.to_owned()
    }
    fn bto(&mut self, value: property::Bto) -> Self {
        self.base.bto = Some(value);
        self.to_owned()
    }
    fn cc(&mut self, value: property::Cc) -> Self {
        self.base.cc = Some(value);
        self.to_owned()
    }
    fn content(&mut self, value: String) -> Self {
        self.base.content = Some(value);
        self.to_owned()
    }
    fn contentMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.contentMap = Some(value);
        self.to_owned()
    }
    fn context(&mut self, value: property::Context) -> Self {
        self.base.context = Some(value);
        self.to_owned()
    }
    fn duration(&mut self, value: String) -> Self {
        self.base.duration = Some(value);
        self.to_owned()
    }
    fn endTime(&mut self, value: String) -> Self {
        self.base.endTime = Some(value);
        self.to_owned()
    }
    fn generator(&mut self, value: property::Generator) -> Self {
        self.base.generator = Some(value);
        self.to_owned()
    }
    fn icon(&mut self, value: property::Icon) -> Self {
        self.base.icon = Some(value);
        self.to_owned()
    }
    fn image(&mut self, value: property::Image) -> Self {
        self.base.image = Some(value);
        self.to_owned()
    }
    fn inReplyTo(&mut self, value: property::InReplyTo) -> Self {
        self.base.inReplyTo = Some(value);
        self.to_owned()
    }
    fn location(&mut self, value: property::Location) -> Self {
        self.base.location = Some(value);
        self.to_owned()
    }
    fn mediaType(&mut self, value: String) -> Self {
        self.base.mediaType = Some(value);
        self.to_owned()
    }
    fn name(&mut self, value: String) -> Self {
        self.base.name = Some(value);
        self.to_owned()
    }
    fn nameMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.nameMap = Some(value);
        self.to_owned()
    }
    fn preview(&mut self, value: property::Preview) -> Self {
        self.base.preview = Some(value);
        self.to_owned()
    }
    fn published(&mut self, value: String) -> Self {
        self.base.published = Some(value);
        self.to_owned()
    }
    fn replies(&mut self, value: property::Replies) -> Self {
        self.base.replies = Some(value);
        self.to_owned()
    }
    fn startTime(&mut self, value: String) -> Self {
        self.base.startTime = Some(value);
        self.to_owned()
    }
    fn summary(&mut self, value: String) -> Self {
        self.base.summary = Some(value);
        self.to_owned()
    }
    fn summaryMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.summaryMap = Some(value);
        self.to_owned()
    }
    fn tag(&mut self, value: property::Tag) -> Self {
        self.base.tag = Some(value);
        self.to_owned()
    }
    fn to(&mut self, value: property::To) -> Self {
        self.base.to = Some(value);
        self.to_owned()
    }
    fn updated(&mut self, value: String) -> Self {
        self.base.updated = Some(value);
        self.to_owned()
    }
    fn url(&mut self, value: property::Url) -> Self {
        self.base.url = Some(value);
        self.to_owned()
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CollectionPage {
    pub partOf: Option<property::PartOf>,
    pub next: Option<property::Next>,
    pub prev: Option<property::Prev>,

    #[serde(flatten)]
    pub base: Object,
}

impl CollectionPage {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("CollectionPage".to_string()))
    }
}

impl StreamTrait for CollectionPage {
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

impl ObjectTrait for CollectionPage {
    fn attachment(&mut self, value: property::Attachment) -> Self {
        self.base.attachment = Some(value);
        self.to_owned()
    }
    fn attributedTo(&mut self, value: property::AttributedTo) -> Self {
        self.base.attributedTo = Some(value);
        self.to_owned()
    }
    fn audience(&mut self, value: property::Audience) -> Self {
        self.base.audience = Some(value);
        self.to_owned()
    }
    fn bcc(&mut self, value: property::Bcc) -> Self {
        self.base.bcc = Some(value);
        self.to_owned()
    }
    fn bto(&mut self, value: property::Bto) -> Self {
        self.base.bto = Some(value);
        self.to_owned()
    }
    fn cc(&mut self, value: property::Cc) -> Self {
        self.base.cc = Some(value);
        self.to_owned()
    }
    fn content(&mut self, value: String) -> Self {
        self.base.content = Some(value);
        self.to_owned()
    }
    fn contentMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.contentMap = Some(value);
        self.to_owned()
    }
    fn context(&mut self, value: property::Context) -> Self {
        self.base.context = Some(value);
        self.to_owned()
    }
    fn duration(&mut self, value: String) -> Self {
        self.base.duration = Some(value);
        self.to_owned()
    }
    fn endTime(&mut self, value: String) -> Self {
        self.base.endTime = Some(value);
        self.to_owned()
    }
    fn generator(&mut self, value: property::Generator) -> Self {
        self.base.generator = Some(value);
        self.to_owned()
    }
    fn icon(&mut self, value: property::Icon) -> Self {
        self.base.icon = Some(value);
        self.to_owned()
    }
    fn image(&mut self, value: property::Image) -> Self {
        self.base.image = Some(value);
        self.to_owned()
    }
    fn inReplyTo(&mut self, value: property::InReplyTo) -> Self {
        self.base.inReplyTo = Some(value);
        self.to_owned()
    }
    fn location(&mut self, value: property::Location) -> Self {
        self.base.location = Some(value);
        self.to_owned()
    }
    fn mediaType(&mut self, value: String) -> Self {
        self.base.mediaType = Some(value);
        self.to_owned()
    }
    fn name(&mut self, value: String) -> Self {
        self.base.name = Some(value);
        self.to_owned()
    }
    fn nameMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.nameMap = Some(value);
        self.to_owned()
    }
    fn preview(&mut self, value: property::Preview) -> Self {
        self.base.preview = Some(value);
        self.to_owned()
    }
    fn published(&mut self, value: String) -> Self {
        self.base.published = Some(value);
        self.to_owned()
    }
    fn replies(&mut self, value: property::Replies) -> Self {
        self.base.replies = Some(value);
        self.to_owned()
    }
    fn startTime(&mut self, value: String) -> Self {
        self.base.startTime = Some(value);
        self.to_owned()
    }
    fn summary(&mut self, value: String) -> Self {
        self.base.summary = Some(value);
        self.to_owned()
    }
    fn summaryMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.summaryMap = Some(value);
        self.to_owned()
    }
    fn tag(&mut self, value: property::Tag) -> Self {
        self.base.tag = Some(value);
        self.to_owned()
    }
    fn to(&mut self, value: property::To) -> Self {
        self.base.to = Some(value);
        self.to_owned()
    }
    fn updated(&mut self, value: String) -> Self {
        self.base.updated = Some(value);
        self.to_owned()
    }
    fn url(&mut self, value: property::Url) -> Self {
        self.base.url = Some(value);
        self.to_owned()
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct OrderedCollectionPage {
    startIndex: Option<u64>,

    // CollectionPage
    pub partOf: Option<property::PartOf>,
    pub next: Option<property::Next>,
    pub prev: Option<property::Prev>,

    #[serde(flatten)]
    pub base: Object,
}

impl OrderedCollectionPage {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("OrderedCollectionPage".to_string()))
    }
}

impl StreamTrait for OrderedCollectionPage {
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

impl ObjectTrait for OrderedCollectionPage {
    fn attachment(&mut self, value: property::Attachment) -> Self {
        self.base.attachment = Some(value);
        self.to_owned()
    }
    fn attributedTo(&mut self, value: property::AttributedTo) -> Self {
        self.base.attributedTo = Some(value);
        self.to_owned()
    }
    fn audience(&mut self, value: property::Audience) -> Self {
        self.base.audience = Some(value);
        self.to_owned()
    }
    fn bcc(&mut self, value: property::Bcc) -> Self {
        self.base.bcc = Some(value);
        self.to_owned()
    }
    fn bto(&mut self, value: property::Bto) -> Self {
        self.base.bto = Some(value);
        self.to_owned()
    }
    fn cc(&mut self, value: property::Cc) -> Self {
        self.base.cc = Some(value);
        self.to_owned()
    }
    fn content(&mut self, value: String) -> Self {
        self.base.content = Some(value);
        self.to_owned()
    }
    fn contentMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.contentMap = Some(value);
        self.to_owned()
    }
    fn context(&mut self, value: property::Context) -> Self {
        self.base.context = Some(value);
        self.to_owned()
    }
    fn duration(&mut self, value: String) -> Self {
        self.base.duration = Some(value);
        self.to_owned()
    }
    fn endTime(&mut self, value: String) -> Self {
        self.base.endTime = Some(value);
        self.to_owned()
    }
    fn generator(&mut self, value: property::Generator) -> Self {
        self.base.generator = Some(value);
        self.to_owned()
    }
    fn icon(&mut self, value: property::Icon) -> Self {
        self.base.icon = Some(value);
        self.to_owned()
    }
    fn image(&mut self, value: property::Image) -> Self {
        self.base.image = Some(value);
        self.to_owned()
    }
    fn inReplyTo(&mut self, value: property::InReplyTo) -> Self {
        self.base.inReplyTo = Some(value);
        self.to_owned()
    }
    fn location(&mut self, value: property::Location) -> Self {
        self.base.location = Some(value);
        self.to_owned()
    }
    fn mediaType(&mut self, value: String) -> Self {
        self.base.mediaType = Some(value);
        self.to_owned()
    }
    fn name(&mut self, value: String) -> Self {
        self.base.name = Some(value);
        self.to_owned()
    }
    fn nameMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.nameMap = Some(value);
        self.to_owned()
    }
    fn preview(&mut self, value: property::Preview) -> Self {
        self.base.preview = Some(value);
        self.to_owned()
    }
    fn published(&mut self, value: String) -> Self {
        self.base.published = Some(value);
        self.to_owned()
    }
    fn replies(&mut self, value: property::Replies) -> Self {
        self.base.replies = Some(value);
        self.to_owned()
    }
    fn startTime(&mut self, value: String) -> Self {
        self.base.startTime = Some(value);
        self.to_owned()
    }
    fn summary(&mut self, value: String) -> Self {
        self.base.summary = Some(value);
        self.to_owned()
    }
    fn summaryMap(&mut self, value: HashMap<String, String>) -> Self {
        self.base.summaryMap = Some(value);
        self.to_owned()
    }
    fn tag(&mut self, value: property::Tag) -> Self {
        self.base.tag = Some(value);
        self.to_owned()
    }
    fn to(&mut self, value: property::To) -> Self {
        self.base.to = Some(value);
        self.to_owned()
    }
    fn updated(&mut self, value: String) -> Self {
        self.base.updated = Some(value);
        self.to_owned()
    }
    fn url(&mut self, value: property::Url) -> Self {
        self.base.url = Some(value);
        self.to_owned()
    }
}
