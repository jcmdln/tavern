use crate::property;
use serde::Serialize;
use serde_json::{Error, Value};
use std::collections::HashMap;

#[allow(non_snake_case)]
pub trait StreamTrait {
    fn atContext(&mut self, value: property::AtContext) -> Self;
    fn id(&mut self, value: String) -> Self;
    fn r#type(&mut self, value: property::Type) -> Self;

    fn to_string(&self) -> Result<String, Error>
    where
        Self: Serialize,
    {
        serde_json::to_string(self)
    }

    fn to_value(&self) -> Result<Value, Error>
    where
        Self: Serialize,
    {
        serde_json::to_value(self)
    }
}

#[allow(non_snake_case)]
pub trait ObjectTrait: StreamTrait {
    fn attachment(&mut self, value: property::Attachment) -> Self;
    fn attributedTo(&mut self, value: property::AttributedTo) -> Self;
    fn audience(&mut self, value: property::Audience) -> Self;
    fn bcc(&mut self, value: property::Bcc) -> Self;
    fn bto(&mut self, value: property::Bto) -> Self;
    fn cc(&mut self, value: property::Cc) -> Self;
    fn content(&mut self, value: String) -> Self;
    fn contentMap(&mut self, value: HashMap<String, String>) -> Self;
    fn context(&mut self, value: property::Context) -> Self;
    fn duration(&mut self, value: String) -> Self;
    fn endTime(&mut self, value: String) -> Self;
    fn generator(&mut self, value: property::Generator) -> Self;
    fn icon(&mut self, value: property::Icon) -> Self;
    fn image(&mut self, value: property::Image) -> Self;
    fn inReplyTo(&mut self, value: property::InReplyTo) -> Self;
    fn location(&mut self, value: property::Location) -> Self;
    fn mediaType(&mut self, value: String) -> Self;
    fn name(&mut self, value: String) -> Self;
    fn nameMap(&mut self, value: HashMap<String, String>) -> Self;
    fn preview(&mut self, value: property::Preview) -> Self;
    fn published(&mut self, value: String) -> Self;
    fn replies(&mut self, value: property::Replies) -> Self;
    fn startTime(&mut self, value: String) -> Self;
    fn summary(&mut self, value: String) -> Self;
    fn summaryMap(&mut self, value: HashMap<String, String>) -> Self;
    fn tag(&mut self, value: property::Tag) -> Self;
    fn to(&mut self, value: property::To) -> Self;
    fn updated(&mut self, value: String) -> Self;
    fn url(&mut self, value: property::Url) -> Self;
}

#[allow(non_snake_case)]
pub trait LinkTrait: StreamTrait {
    fn href(&mut self, value: String) -> Self;
    fn rel(&mut self, value: property::Rel) -> Self;
    fn mediaType(&mut self, value: String) -> Self;
    fn name(&mut self, value: String) -> Self;
    fn hreflang(&mut self, value: String) -> Self;
    fn height(&mut self, value: u64) -> Self;
    fn width(&mut self, value: u64) -> Self;
    fn preview(&mut self, value: property::Preview) -> Self;
}

#[allow(non_snake_case)]
pub trait ActivityTrait: IntransitiveActivityTrait {
    fn object(&mut self, value: property::Object) -> Self;
}

#[allow(non_snake_case)]
pub trait IntransitiveActivityTrait: ObjectTrait {
    fn actor(&mut self, value: property::Actor) -> Self;
    fn target(&mut self, value: property::Target) -> Self;
    fn result(&mut self, value: property::Result) -> Self;
    fn origin(&mut self, value: property::Origin) -> Self;
    fn instrument(&mut self, value: property::Instrument) -> Self;
}

#[allow(non_snake_case)]
pub trait CollectionTrait {
    fn totalItems(&self, value: u64) -> Self;
    fn current(&mut self, value: property::Current) -> Self;
    fn first(&mut self, value: property::First) -> Self;
    fn last(&mut self, value: property::Last) -> Self;
    fn items(&mut self, value: property::Items) -> Self;
}

#[allow(non_snake_case)]
pub trait OrderedCollectionTrait: CollectionTrait {}

#[allow(non_snake_case)]
pub trait CollectionPageTrait: CollectionTrait {
    fn partOf(&mut self, value: Value) -> Self;
    fn next(&mut self, value: Value) -> Self;
    fn prev(&mut self, value: Value) -> Self;
}

#[allow(non_snake_case)]
pub trait OrderedCollectionPageTrait: OrderedCollectionTrait + CollectionPageTrait {
    fn startIndex(&self, value: u64) -> Self;
}
