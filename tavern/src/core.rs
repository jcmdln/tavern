use crate::{
    property,
    traits::{
        CollectionPageTrait, CollectionTrait, LinkTrait, ObjectTrait, OrderedCollectionPageTrait,
        OrderedCollectionTrait, StreamTrait,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Everything is a `Stream` which inherits base properties that belong to all
/// `Object`s and `Link`s.
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Stream {
    #[serde(rename = "@context")]
    pub atContext: Option<property::AtContext>,
    pub id: Option<String>,
    pub r#type: Option<property::Type>,
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Object {
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

    #[serde(flatten)]
    pub extends: Stream,
}

impl StreamTrait for Object {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Object".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.extends
    }
}

impl ObjectTrait for Object {
    fn as_object(&mut self) -> &mut crate::core::Object {
        self
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Link {
    pub href: Option<String>,
    pub hreflang: Option<String>,
    pub mediaType: Option<String>,
    pub name: Option<String>,
    pub rel: Option<property::Rel>,
    pub preview: Option<property::Preview>,
    pub height: Option<u64>,
    pub width: Option<u64>,

    #[serde(flatten)]
    pub extends: Stream,
}

impl StreamTrait for Link {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Link".to_string()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.extends
    }
}

impl LinkTrait for Link {
    fn as_link(&mut self) -> &mut Self {
        self
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
    pub extends: Object,
}

impl StreamTrait for Activity {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Activity".to_string()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Activity {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct IntransitiveActivity {
    pub actor: Option<property::Actor>,
    pub target: Option<property::Target>,
    pub result: Option<property::Result>,
    pub origin: Option<property::Origin>,
    pub instrument: Option<property::Instrument>,

    #[serde(flatten)]
    pub extends: Object,
}

impl StreamTrait for IntransitiveActivity {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("IntransitiveActivity".to_string()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for IntransitiveActivity {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
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
    pub extends: Object,
}

impl StreamTrait for Collection {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Collection".to_string()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Collection {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.extends
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
    pub extends: Collection,
}

impl StreamTrait for OrderedCollection {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("OrderedCollection".to_string()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_collection().as_object().extends
    }
}

impl CollectionTrait for OrderedCollection {
    fn as_collection(&mut self) -> &mut crate::core::Collection {
        &mut self.extends
    }
}

impl OrderedCollectionTrait for OrderedCollection {
    fn as_ordered_collection(&mut self) -> &mut crate::core::OrderedCollection {
        self
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
    pub extends: Box<Collection>,
}

impl StreamTrait for CollectionPage {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("CollectionPage".to_string()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for CollectionPage {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.as_collection().extends
    }
}

impl CollectionTrait for CollectionPage {
    fn as_collection(&mut self) -> &mut crate::core::Collection {
        &mut self.as_collection_page().extends
    }
}

impl CollectionPageTrait for CollectionPage {
    fn as_collection_page(&mut self) -> &mut crate::core::CollectionPage {
        self
    }
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct OrderedCollectionPage {
    pub startIndex: Option<u64>,

    #[serde(flatten)]
    pub extends: CollectionPage,
}

impl StreamTrait for OrderedCollectionPage {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("OrderedCollectionPage".to_string()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for OrderedCollectionPage {
    fn as_object(&mut self) -> &mut crate::core::Object {
        &mut self.as_collection().extends
    }
}

impl CollectionTrait for OrderedCollectionPage {
    fn as_collection(&mut self) -> &mut crate::core::Collection {
        &mut self.as_collection_page().extends
    }
}

impl CollectionPageTrait for OrderedCollectionPage {
    fn as_collection_page(&mut self) -> &mut crate::core::CollectionPage {
        &mut self.extends
    }
}

impl OrderedCollectionPageTrait for OrderedCollectionPage {
    fn as_ordered_collection_page(&mut self) -> &mut crate::core::OrderedCollectionPage {
        self
    }
}
