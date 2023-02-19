use crate::property;
use serde::Serialize;
use serde_json::{Error, Value};
use std::collections::HashMap;

pub trait StreamTrait {
    fn as_stream(&mut self) -> &mut crate::core::Stream;

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

    fn at_context(&mut self, value: property::AtContext) -> &mut Self {
        self.as_stream().at_context = Some(value);
        self
    }
    fn id(&mut self, value: String) -> &mut Self {
        let mut stream = self.as_stream();
        stream.id = Some(value);
        self
    }
    fn kind(&mut self, value: property::Type) -> &mut Self {
        let mut stream = self.as_stream();
        stream.kind = Some(value);
        self
    }
}

pub trait ObjectTrait: StreamTrait {
    fn as_object(&mut self) -> &mut crate::core::Object;

    fn attachment(&mut self, value: property::Attachment) -> &mut Self {
        self.as_object().attachment = Some(value);
        self
    }
    fn attributed_to(&mut self, value: property::AttributedTo) -> &mut Self {
        self.as_object().attributed_to = Some(value);
        self
    }
    fn audience(&mut self, value: property::Audience) -> &mut Self {
        self.as_object().audience = Some(value);
        self
    }
    fn bcc(&mut self, value: property::Bcc) -> &mut Self {
        self.as_object().bcc = Some(value);
        self
    }
    fn bto(&mut self, value: property::Bto) -> &mut Self {
        self.as_object().bto = Some(value);
        self
    }
    fn cc(&mut self, value: property::Cc) -> &mut Self {
        self.as_object().cc = Some(value);
        self
    }
    fn content(&mut self, value: String) -> &mut Self {
        self.as_object().content = Some(value);
        self
    }
    fn content_map(&mut self, value: HashMap<String, String>) -> &mut Self {
        self.as_object().content_map = Some(value);
        self
    }
    fn context(&mut self, value: property::Context) -> &mut Self {
        self.as_object().context = Some(value);
        self
    }
    fn duration(&mut self, value: String) -> &mut Self {
        self.as_object().duration = Some(value);
        self
    }
    fn end_time(&mut self, value: String) -> &mut Self {
        self.as_object().end_time = Some(value);
        self
    }
    fn generator(&mut self, value: property::Generator) -> &mut Self {
        self.as_object().generator = Some(value);
        self
    }
    fn icon(&mut self, value: property::Icon) -> &mut Self {
        self.as_object().icon = Some(value);
        self
    }
    fn image(&mut self, value: property::Image) -> &mut Self {
        self.as_object().image = Some(value);
        self
    }
    fn in_reply_to(&mut self, value: property::InReplyTo) -> &mut Self {
        self.as_object().in_reply_to = Some(value);
        self
    }
    fn location(&mut self, value: property::Location) -> &mut Self {
        self.as_object().location = Some(value);
        self
    }
    fn media_type(&mut self, value: String) -> &mut Self {
        self.as_object().media_type = Some(value);
        self
    }
    fn name(&mut self, value: String) -> &mut Self {
        self.as_object().name = Some(value);
        self
    }
    fn name_map(&mut self, value: HashMap<String, String>) -> &mut Self {
        self.as_object().name_map = Some(value);
        self
    }
    fn preview(&mut self, value: property::Preview) -> &mut Self {
        self.as_object().preview = Some(value);
        self
    }
    fn published(&mut self, value: String) -> &mut Self {
        self.as_object().published = Some(value);
        self
    }
    fn replies(&mut self, value: property::Replies) -> &mut Self {
        self.as_object().replies = Some(value);
        self
    }
    fn start_time(&mut self, value: String) -> &mut Self {
        self.as_object().start_time = Some(value);
        self
    }
    fn summary(&mut self, value: String) -> &mut Self {
        self.as_object().summary = Some(value);
        self
    }
    fn summary_map(&mut self, value: HashMap<String, String>) -> &mut Self {
        self.as_object().summary_map = Some(value);
        self
    }
    fn tag(&mut self, value: property::Tag) -> &mut Self {
        self.as_object().tag = Some(value);
        self
    }
    fn to(&mut self, value: property::To) -> &mut Self {
        self.as_object().to = Some(value);
        self
    }
    fn updated(&mut self, value: String) -> &mut Self {
        self.as_object().updated = Some(value);
        self
    }
    fn url(&mut self, value: property::Url) -> &mut Self {
        self.as_object().url = Some(value);
        self
    }
}

pub trait LinkTrait: StreamTrait {
    fn as_link(&mut self) -> &mut crate::core::Link;

    fn href(&mut self, value: String) -> &mut Self {
        self.as_link().href = Some(value);
        self
    }
    fn rel(&mut self, value: property::Rel) -> &mut Self {
        self.as_link().rel = Some(value);
        self
    }
    fn media_type(&mut self, value: String) -> &mut Self {
        self.as_link().media_type = Some(value);
        self
    }
    fn name(&mut self, value: String) -> &mut Self {
        self.as_link().name = Some(value);
        self
    }
    fn hreflang(&mut self, value: String) -> &mut Self {
        self.as_link().hreflang = Some(value);
        self
    }
    fn height(&mut self, value: u64) -> &mut Self {
        self.as_link().height = Some(value);
        self
    }
    fn width(&mut self, value: u64) -> &mut Self {
        self.as_link().width = Some(value);
        self
    }
    fn preview(&mut self, value: property::Preview) -> &mut Self {
        self.as_link().preview = Some(value);
        self
    }
}

pub trait ActivityTrait: ObjectTrait {
    fn as_activity(&mut self) -> &mut crate::core::Activity;

    fn actor(&mut self, value: property::Actor) -> &mut Self {
        self.as_activity().actor = Some(value);
        self
    }
    fn object(&mut self, value: property::Object) -> &mut Self {
        self.as_activity().object = Some(value);
        self
    }
    fn target(&mut self, value: property::Target) -> &mut Self {
        self.as_activity().target = Some(value);
        self
    }
    fn result(&mut self, value: property::Result) -> &mut Self {
        self.as_activity().result = Some(value);
        self
    }
    fn origin(&mut self, value: property::Origin) -> &mut Self {
        self.as_activity().origin = Some(value);
        self
    }
    fn instrument(&mut self, value: property::Instrument) -> &mut Self {
        self.as_activity().instrument = Some(value);
        self
    }
}

pub trait IntransitiveActivityTrait: ObjectTrait {
    fn as_intransitive_activity(&mut self) -> &mut crate::core::IntransitiveActivity;

    fn actor(&mut self, value: property::Actor) -> &mut Self {
        self.as_intransitive_activity().actor = Some(value);
        self
    }
    fn target(&mut self, value: property::Target) -> &mut Self {
        self.as_intransitive_activity().target = Some(value);
        self
    }
    fn result(&mut self, value: property::Result) -> &mut Self {
        self.as_intransitive_activity().result = Some(value);
        self
    }
    fn origin(&mut self, value: property::Origin) -> &mut Self {
        self.as_intransitive_activity().origin = Some(value);
        self
    }
    fn instrument(&mut self, value: property::Instrument) -> &mut Self {
        self.as_intransitive_activity().instrument = Some(value);
        self
    }
}

pub trait CollectionTrait {
    fn as_collection(&mut self) -> &mut crate::core::Collection;

    fn total_items(&mut self, value: u64) -> &mut Self {
        self.as_collection().total_items = Some(value);
        self
    }
    fn current(&mut self, value: property::Current) -> &mut Self {
        self.as_collection().current = Some(value);
        self
    }
    fn first(&mut self, value: property::First) -> &mut Self {
        self.as_collection().first = Some(value);
        self
    }
    fn last(&mut self, value: property::Last) -> &mut Self {
        self.as_collection().last = Some(value);
        self
    }
    fn items(&mut self, value: property::Items) -> &mut Self {
        self.as_collection().items = Some(value);
        self
    }
}

pub trait OrderedCollectionTrait: CollectionTrait {
    fn as_ordered_collection(&mut self) -> &mut crate::core::OrderedCollection;
}

pub trait CollectionPageTrait: CollectionTrait {
    fn as_collection_page(&mut self) -> &mut crate::core::CollectionPage;

    fn part_of(&mut self, value: property::PartOf) -> &mut Self {
        self.as_collection_page().part_of = Some(value);
        self
    }
    fn next(&mut self, value: property::Next) -> &mut Self {
        self.as_collection_page().next = Some(value);
        self
    }
    fn prev(&mut self, value: property::Prev) -> &mut Self {
        self.as_collection_page().prev = Some(value);
        self
    }
}

pub trait OrderedCollectionPageTrait: CollectionPageTrait {
    fn as_ordered_collection_page(&mut self) -> &mut crate::core::OrderedCollectionPage;

    fn start_index(&mut self, value: u64) -> &mut Self {
        self.as_ordered_collection_page().start_index = Some(value);
        self
    }
}
