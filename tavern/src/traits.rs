pub use serde_json::{json, Value};

pub trait StreamTrait {
    fn at_context(&mut self, value: Value) -> Self;
    fn r#type(&mut self, value: Value) -> Self;
}

#[allow(non_snake_case)]
pub trait ObjectTrait: StreamTrait {
    fn attachment(&mut self, value: Value) -> Self;
    fn attributedTo(&mut self, value: Value) -> Self;
    fn audience(&mut self, value: Value) -> Self;
    fn bcc(&mut self, value: Value) -> Self;
    fn bto(&mut self, value: Value) -> Self;
    fn cc(&mut self, value: Value) -> Self;
    fn content(&mut self, value: String) -> Self;
    fn contentMap(&mut self, value: Value) -> Self;
    fn context(&mut self, value: Value) -> Self;
    fn duration(&mut self, value: f32) -> Self;
    fn endTime(&mut self, value: String) -> Self;
    fn generator(&mut self, value: Value) -> Self;
    fn icon(&mut self, value: Value) -> Self;
    fn image(&mut self, value: Value) -> Self;
    fn inReplyTo(&mut self, value: Value) -> Self;
    fn location(&mut self, value: Value) -> Self;
    fn mediaType(&mut self, value: Value) -> Self;
    fn name(&mut self, value: String) -> Self;
    fn nameMap(&mut self, value: Value) -> Self;
    fn preview(&mut self, value: Value) -> Self;
    fn published(&mut self, value: String) -> Self;
    fn replies(&mut self, value: Value) -> Self;
    fn startTime(&mut self, value: String) -> Self;
    fn summary(&mut self, value: String) -> Self;
    fn summaryMap(&mut self, value: Value) -> Self;
    fn tag(&mut self, value: Value) -> Self;
    fn to(&mut self, value: Value) -> Self;
    fn updated(&mut self, value: Value) -> Self;
    fn url(&mut self, value: Value) -> Self;
}

#[allow(non_snake_case)]
pub trait LinkTrait: StreamTrait {
    fn href(&mut self, value: Value) -> Self;
    fn rel(&mut self, value: Value) -> Self;
    fn mediaType(&mut self, value: Value) -> Self;
    fn name(&mut self, value: Value) -> Self;
    fn hreflang(&mut self, value: String) -> String;
    fn height(&mut self, value: u32) -> u32;
    fn width(&mut self, value: u32) -> u32;
    fn preview(&mut self, value: Value) -> Self;
}

#[allow(non_snake_case)]
pub trait ActivityTrait: IntransitiveActivityTrait {
    fn object(&mut self, value: Value) -> Self;
}

#[allow(non_snake_case)]
pub trait IntransitiveActivityTrait: ObjectTrait {
    fn actor(&mut self, value: Value) -> Self;
    fn target(&mut self, value: Value) -> Self;
    fn result(&mut self, value: Value) -> Self;
    fn origin(&mut self, value: Value) -> Self;
    fn instrument(&mut self, value: Value) -> Self;
}

#[allow(non_snake_case)]
pub trait CollectionTrait {
    fn totalItems(&self) -> Option<u64>;
    fn current(&mut self, value: Value) -> Self;
    fn first(&mut self, value: Value) -> Self;
    fn last(&mut self, value: Value) -> Self;
    fn items(&mut self, value: Value) -> Self;
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
    fn startIndex(&self) -> Option<u64>;
}
