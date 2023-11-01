use crate::{
    core, property,
    traits::{
        CollectionPageTrait, CollectionTrait, LinkTrait, ObjectTrait, OrderedCollectionPageTrait,
        OrderedCollectionTrait, StreamTrait,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Everything is a `Stream` which inherits base properties that belong to all
/// `Object`s and `Link`s.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Stream {
    pub id: Option<String>,
    #[serde(rename = "@context")]
    pub at_context: Option<property::AtContext>,
    #[serde(rename = "type")]
    pub kind: Option<property::Type>,
}

impl Default for Stream {
    fn default() -> Self {
        Self {
            at_context: Some(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".into(),
            )),
            id: None,
            kind: None,
        }
    }
}

impl StreamTrait for Stream {
    fn as_stream(&mut self) -> &mut Stream {
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    #[serde(flatten)]
    pub extends: Stream,

    pub to: Option<property::To>,
    pub bto: Option<property::Bto>,
    pub cc: Option<property::Cc>,
    pub bcc: Option<property::Bcc>,

    pub attachment: Option<property::Attachment>,
    pub attributed_to: Option<property::AttributedTo>,
    pub audience: Option<property::Audience>,
    pub content_map: Option<HashMap<String, String>>,
    pub content: Option<String>,
    pub context: Option<property::Context>,
    pub duration: Option<String>,
    pub end_time: Option<String>,
    pub generator: Option<property::Generator>,
    pub icon: Option<property::Icon>,
    pub image: Option<property::Image>,
    pub in_reply_to: Option<property::InReplyTo>,
    pub location: Option<property::Location>,
    pub media_type: Option<String>,
    pub name_map: Option<HashMap<String, String>>,
    pub name: Option<String>,
    pub preview: Option<property::Preview>,
    pub published: Option<String>,
    pub replies: Option<property::Replies>,
    pub start_time: Option<String>,
    pub summary_map: Option<HashMap<String, String>>,
    pub summary: Option<String>,
    pub tag: Option<property::Tag>,
    pub updated: Option<String>,
    pub url: Option<property::Url>,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            extends: Stream::default().kind(property::Type::String("Object".into())).clone(),
            to: None,
            bto: None,
            cc: None,
            bcc: None,
            attachment: None,
            attributed_to: None,
            audience: None,
            content_map: None,
            content: None,
            context: None,
            duration: None,
            end_time: None,
            generator: None,
            icon: None,
            image: None,
            in_reply_to: None,
            location: None,
            media_type: None,
            name_map: None,
            name: None,
            preview: None,
            published: None,
            replies: None,
            start_time: None,
            summary_map: None,
            summary: None,
            tag: None,
            updated: None,
            url: None,
        }
    }
}

impl StreamTrait for Object {
    fn as_stream(&mut self) -> &mut Stream {
        &mut self.extends
    }
}

impl ObjectTrait for Object {
    fn as_object(&mut self) -> &mut Object {
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    #[serde(flatten)]
    pub extends: Stream,

    pub href: Option<String>,
    pub hreflang: Option<String>,
    pub media_type: Option<String>,
    pub name: Option<String>,
    pub rel: Option<property::Rel>,
    pub preview: Option<property::Preview>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            extends: Stream::default().kind(property::Type::String("Link".into())).clone(),
            href: None,
            hreflang: None,
            media_type: None,
            name: None,
            rel: None,
            preview: None,
            height: None,
            width: None,
        }
    }
}

impl StreamTrait for Link {
    fn as_stream(&mut self) -> &mut Stream {
        &mut self.extends
    }
}

impl LinkTrait for Link {
    fn as_link(&mut self) -> &mut Self {
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Activity {
    #[serde(flatten)]
    pub extends: Object,

    pub actor: Option<property::Actor>,
    pub object: Option<property::Object>,
    pub target: Option<property::Target>,
    pub result: Option<property::Result>,
    pub origin: Option<property::Origin>,
    pub instrument: Option<property::Instrument>,
}

impl Default for Activity {
    fn default() -> Self {
        Self {
            extends: Object::default().kind(property::Type::String("Activity".into())).clone(),
            actor: None,
            object: None,
            target: None,
            result: None,
            origin: None,
            instrument: None,
        }
    }
}

impl StreamTrait for Activity {
    fn as_stream(&mut self) -> &mut Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Activity {
    fn as_object(&mut self) -> &mut Object {
        &mut self.extends
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IntransitiveActivity {
    #[serde(flatten)]
    pub extends: Object,

    pub actor: Option<property::Actor>,
    pub target: Option<property::Target>,
    pub result: Option<property::Result>,
    pub origin: Option<property::Origin>,
    pub instrument: Option<property::Instrument>,
}

impl Default for IntransitiveActivity {
    fn default() -> Self {
        Self {
            extends: Object::default()
                .kind(property::Type::String("IntransitiveActivity".into()))
                .clone(),
            actor: None,
            target: None,
            result: None,
            origin: None,
            instrument: None,
        }
    }
}

impl StreamTrait for IntransitiveActivity {
    fn as_stream(&mut self) -> &mut Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for IntransitiveActivity {
    fn as_object(&mut self) -> &mut Object {
        &mut self.extends
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(flatten)]
    pub extends: Object,

    pub total_items: Option<u64>,
    pub current: Option<property::Current>,
    pub first: Option<property::First>,
    pub last: Option<property::Last>,
    pub items: Option<property::Items>,
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            extends: Object::default().kind(property::Type::String("Collection".into())).clone(),
            total_items: None,
            current: None,
            first: None,
            last: None,
            items: None,
        }
    }
}

impl StreamTrait for Collection {
    fn as_stream(&mut self) -> &mut Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Collection {
    fn as_object(&mut self) -> &mut Object {
        &mut self.extends
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollection {
    #[serde(flatten)]
    pub extends: Collection,
}

impl Default for OrderedCollection {
    fn default() -> Self {
        Self {
            extends: Collection::default()
                .kind(property::Type::String("OrderedCollection".into()))
                .clone(),
        }
    }
}

impl StreamTrait for OrderedCollection {
    fn as_stream(&mut self) -> &mut Stream {
        &mut self.as_collection().as_object().extends
    }
}

impl CollectionTrait for OrderedCollection {
    fn as_collection(&mut self) -> &mut Collection {
        &mut self.extends
    }
}

impl OrderedCollectionTrait for OrderedCollection {
    fn as_ordered_collection(&mut self) -> &mut OrderedCollection {
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPage {
    #[serde(flatten)]
    pub extends: Box<Collection>,

    pub part_of: Option<property::PartOf>,
    pub next: Option<property::Next>,
    pub prev: Option<property::Prev>,
}

impl Default for CollectionPage {
    fn default() -> Self {
        Self {
            extends: Box::new(
                core::Collection::default()
                    .kind(property::Type::String("CollectionPage".into()))
                    .clone(),
            ),
            part_of: None,
            next: None,
            prev: None,
        }
    }
}

impl StreamTrait for CollectionPage {
    fn as_stream(&mut self) -> &mut Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for CollectionPage {
    fn as_object(&mut self) -> &mut Object {
        &mut self.as_collection().extends
    }
}

impl CollectionTrait for CollectionPage {
    fn as_collection(&mut self) -> &mut Collection {
        &mut self.as_collection_page().extends
    }
}

impl CollectionPageTrait for CollectionPage {
    fn as_collection_page(&mut self) -> &mut CollectionPage {
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPage {
    #[serde(flatten)]
    pub extends: CollectionPage,

    pub start_index: Option<u64>,
}

impl Default for OrderedCollectionPage {
    fn default() -> Self {
        Self {
            extends: core::CollectionPage::default()
                .kind(property::Type::String("OrderedCollectionPage".into()))
                .clone(),
            start_index: None,
        }
    }
}

impl StreamTrait for OrderedCollectionPage {
    fn as_stream(&mut self) -> &mut Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for OrderedCollectionPage {
    fn as_object(&mut self) -> &mut Object {
        &mut self.as_collection().extends
    }
}

impl CollectionTrait for OrderedCollectionPage {
    fn as_collection(&mut self) -> &mut Collection {
        &mut self.as_collection_page().extends
    }
}

impl CollectionPageTrait for OrderedCollectionPage {
    fn as_collection_page(&mut self) -> &mut CollectionPage {
        &mut self.extends
    }
}

impl OrderedCollectionPageTrait for OrderedCollectionPage {
    fn as_ordered_collection_page(&mut self) -> &mut OrderedCollectionPage {
        self
    }
}
