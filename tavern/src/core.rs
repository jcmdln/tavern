use crate::traits::StreamTrait;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

pub enum Stream {
    Object(Object),
    Link(Link),
    Activity(Activity),
    IntransitiveActivity(IntransitiveActivity),
    Collection(Collection),
    OrderedCollection(OrderedCollection),
    CollectionPage(CollectionPage),
    OrderedCollectionPage(OrderedCollectionPage),
    Vec(Vec<Stream>),
}

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Object {
    #[serde(rename = "@context")]
    pub at_context: Value,
    pub r#type: Value,
    pub attachment: Option<Value>,
    pub attributedTo: Option<Value>,
    pub audience: Option<Value>,
    pub bcc: Option<Value>,
    pub bto: Option<Value>,
    pub cc: Option<Value>,
    pub content: Option<String>,
    pub contentMap: Option<Value>,
    pub context: Option<Value>,
    pub duration: Option<f32>,
    pub endTime: Option<String>,
    pub generator: Option<Value>,
    pub icon: Option<Value>,
    pub image: Option<Value>,
    pub inReplyTo: Option<Value>,
    pub location: Option<Value>,
    pub mediaType: Option<Value>,
    pub name: Option<String>,
    pub nameMap: Option<Value>,
    pub preview: Option<Value>,
    pub published: Option<String>,
    pub replies: Option<Value>,
    pub startTime: Option<String>,
    pub summary: Option<String>,
    pub summaryMap: Option<Value>,
    pub tag: Option<Value>,
    pub to: Option<Value>,
    pub updated: Option<Value>,
    pub url: Option<Value>,
}

impl Object {
    pub fn new() -> Object {
        Object::default()
            .at_context(json!("https://www.w3.org/ns/activitystreams"))
            .r#type(json!("Object"))
    }
}

impl StreamTrait for Object {
    fn at_context(&mut self, value: Value) -> Self {
        self.at_context = value;
        self.to_owned()
    }
    fn r#type(&mut self, value: Value) -> Self {
        self.r#type = value;
        self.to_owned()
    }
}

// impl ObjectTrait for Object {
//     fn attachment(&mut self, value: Value) -> Self {
//         self.attachment = Some(value);
//         self.to_owned()
//     }
//     fn attributedTo(&mut self, value: Value) -> Self {
//         self.attributedTo = Some(value);
//         self.to_owned()
//     }
//     fn audience(&mut self, value: Value) -> Self {
//         self.audience = Some(value);
//         self.to_owned()
//     }
//     fn bcc(&mut self, value: Value) -> Self {
//         self.bcc = Some(value);
//         self.to_owned()
//     }
//     fn bto(&mut self, value: Value) -> Self {
//         self.bto = Some(value);
//         self.to_owned()
//     }
//     fn cc(&mut self, value: Value) -> Self {
//         self.cc = Some(value);
//         self.to_owned()
//     }
//     fn content(&mut self, value: String) -> Self {
//         self.content = Some(value);
//         self.to_owned()
//     }
//     fn contentMap(&mut self, value: Value) -> Self {
//         self.contentMap = Some(value);
//         self.to_owned()
//     }
//     fn context(&mut self, value: Value) -> Self {
//         self.context = Some(value);
//         self.to_owned()
//     }
//     fn duration(&mut self, value: f32) -> Self {
//         self.duration = Some(value);
//         self.to_owned()
//     }
//     fn endTime(&mut self, value: String) -> Self {
//         self.endTime = Some(value);
//         self.to_owned()
//     }
//     fn generator(&mut self, value: Value) -> Self {
//         self.generator = Some(value);
//         self.to_owned()
//     }
//     fn icon(&mut self, value: Value) -> Self {
//         self.icon = Some(value);
//         self.to_owned()
//     }
//     fn image(&mut self, value: Value) -> Self {
//         self.image = Some(value);
//         self.to_owned()
//     }
//     fn inReplyTo(&mut self, value: Value) -> Self {
//         self.inReplyTo = Some(value);
//         self.to_owned()
//     }
//     fn location(&mut self, value: Value) -> Self {
//         self.location = Some(value);
//         self.to_owned()
//     }
//     fn mediaType(&mut self, value: Value) -> Self {
//         self.mediaType = Some(value);
//         self.to_owned()
//     }
//     fn name(&mut self, value: String) -> Self {
//         self.name = Some(value);
//         self.to_owned()
//     }
//     fn nameMap(&mut self, value: Value) -> Self {
//         self.nameMap = Some(value);
//         self.to_owned()
//     }
//     fn preview(&mut self, value: Value) -> Self {
//         self.preview = Some(value);
//         self.to_owned()
//     }
//     fn published(&mut self, value: String) -> Self {
//         self.published = Some(value);
//         self.to_owned()
//     }
//     fn replies(&mut self, value: Value) -> Self {
//         self.replies = Some(value);
//         self.to_owned()
//     }
//     fn startTime(&mut self, value: String) -> Self {
//         self.startTime = Some(value);
//         self.to_owned()
//     }
//     fn summary(&mut self, value: String) -> Self {
//         self.summary = Some(value);
//         self.to_owned()
//     }
//     fn summaryMap(&mut self, value: Value) -> Self {
//         self.summaryMap = Some(value);
//         self.to_owned()
//     }
//     fn tag(&mut self, value: Value) -> Self {
//         self.tag = Some(value);
//         self.to_owned()
//     }
//     fn to(&mut self, value: Value) -> Self {
//         self.to = Some(value);
//         self.to_owned()
//     }
//     fn updated(&mut self, value: Value) -> Self {
//         self.updated = Some(value);
//         self.to_owned()
//     }
//     fn url(&mut self, value: Value) -> Self {
//         self.url = Some(value);
//         self.to_owned()
//     }
// }

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Link {
    #[serde(rename = "@context")]
    pub at_context: Value,
    pub r#type: Value,
    pub href: Option<String>,
    pub hreflang: Option<String>,
    pub mediaType: Option<String>,
    pub name: Option<String>,
    pub rel: Option<String>,
    pub preview: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

impl Link {
    pub fn new() -> Link {
        Link::default()
            .at_context(json!("https://www.w3.org/ns/activitystreams"))
            .r#type(json!("Link"))
    }
}

impl StreamTrait for Link {
    fn at_context(&mut self, value: Value) -> Self {
        self.at_context = value;
        self.to_owned()
    }
    fn r#type(&mut self, value: Value) -> Self {
        self.r#type = value;
        self.to_owned()
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Activity {
    pub actor: Option<Value>,
    pub object: Option<Value>,
    pub target: Option<Value>,
    pub result: Option<Value>,
    pub origin: Option<Value>,
    pub instrument: Option<Value>,

    #[serde(flatten)]
    pub base: Object,
}

impl Activity {
    pub fn new() -> Activity {
        Activity::default()
            .at_context(json!("https://www.w3.org/ns/activitystreams"))
            .r#type(json!("Activity"))
    }
}

impl StreamTrait for Activity {
    fn at_context(&mut self, value: Value) -> Self {
        self.base.at_context = value;
        self.to_owned()
    }
    fn r#type(&mut self, value: Value) -> Self {
        self.base.r#type = value;
        self.to_owned()
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct IntransitiveActivity {
    pub anyOf: Option<Value>,
    pub closed: Option<Value>,
    pub oneOf: Option<Value>,

    #[serde(flatten)]
    pub base: Object,
}

impl IntransitiveActivity {
    pub fn new() -> IntransitiveActivity {
        IntransitiveActivity::default()
            .at_context(json!("https://www.w3.org/ns/activitystreams"))
            .r#type(json!("IntransitiveActivity"))
    }
}

impl StreamTrait for IntransitiveActivity {
    fn at_context(&mut self, value: Value) -> Self {
        self.base.at_context = value;
        self.to_owned()
    }
    fn r#type(&mut self, value: Value) -> Self {
        self.base.r#type = value;
        self.to_owned()
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Collection {
    #[serde(flatten)]
    pub base: Object,
}

impl Collection {
    pub fn new() -> Collection {
        Collection::default()
            .at_context(json!("https://www.w3.org/ns/activitystreams"))
            .r#type(json!("Collection"))
    }
}

impl StreamTrait for Collection {
    fn at_context(&mut self, value: Value) -> Self {
        self.base.at_context = value;
        self.to_owned()
    }
    fn r#type(&mut self, value: Value) -> Self {
        self.base.r#type = value;
        self.to_owned()
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct OrderedCollection {
    #[serde(flatten)]
    pub base: Object,
}

impl OrderedCollection {
    pub fn new() -> OrderedCollection {
        OrderedCollection::default()
            .at_context(json!("https://www.w3.org/ns/activitystreams"))
            .r#type(json!("OrderedCollection"))
    }
}

impl StreamTrait for OrderedCollection {
    fn at_context(&mut self, value: Value) -> Self {
        self.base.at_context = value;
        self.to_owned()
    }
    fn r#type(&mut self, value: Value) -> Self {
        self.base.r#type = value;
        self.to_owned()
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CollectionPage {
    #[serde(flatten)]
    pub base: Object,
}

impl CollectionPage {
    pub fn new() -> CollectionPage {
        CollectionPage::default()
            .at_context(json!("https://www.w3.org/ns/activitystreams"))
            .r#type(json!("CollectionPage"))
    }
}

impl StreamTrait for CollectionPage {
    fn at_context(&mut self, value: Value) -> Self {
        self.base.at_context = value;
        self.to_owned()
    }
    fn r#type(&mut self, value: Value) -> Self {
        self.base.r#type = value;
        self.to_owned()
    }
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct OrderedCollectionPage {
    #[serde(flatten)]
    pub base: Object,
}

impl OrderedCollectionPage {
    pub fn new() -> OrderedCollectionPage {
        OrderedCollectionPage::default()
            .at_context(json!("https://www.w3.org/ns/activitystreams"))
            .r#type(json!("OrderedCollectionPage"))
    }
}

impl StreamTrait for OrderedCollectionPage {
    fn at_context(&mut self, value: Value) -> Self {
        self.base.at_context = value;
        self.to_owned()
    }
    fn r#type(&mut self, value: Value) -> Self {
        self.base.r#type = value;
        self.to_owned()
    }
}
