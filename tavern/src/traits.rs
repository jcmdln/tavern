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

#[macro_export]
macro_rules! impl_Object_for {
    ($t:ident) => {
        #[allow(non_snake_case)]
        impl $crate::traits::StreamTrait for $t {
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

        #[allow(non_snake_case)]
        impl $crate::traits::ObjectTrait for $t {
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
            fn contentMap(&mut self, value: std::collections::HashMap<String, String>) -> Self {
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
            fn nameMap(&mut self, value: std::collections::HashMap<String, String>) -> Self {
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
            fn summaryMap(&mut self, value: std::collections::HashMap<String, String>) -> Self {
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
    };
}
