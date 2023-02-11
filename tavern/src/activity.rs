use crate::{core::Object, property, traits::StreamTrait};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Accept {
    #[serde(flatten)]
    pub base: Object,
}

impl Accept {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Accept".to_string()))
    }
}

impl StreamTrait for Accept {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Add {
    #[serde(flatten)]
    pub base: Object,
}

impl Add {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Add".to_string()))
    }
}

impl StreamTrait for Add {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Announce {
    #[serde(flatten)]
    pub base: Object,
}

impl Announce {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Announce".to_string()))
    }
}

impl StreamTrait for Announce {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Arrive {
    #[serde(flatten)]
    pub base: Object,
}

impl Arrive {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Arrive".to_string()))
    }
}

impl StreamTrait for Arrive {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Block {
    #[serde(flatten)]
    pub base: Object,
}

impl Block {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Block".to_string()))
    }
}

impl StreamTrait for Block {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Create {
    #[serde(flatten)]
    pub base: Object,
}

impl Create {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Create".to_string()))
    }
}

impl StreamTrait for Create {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Delete {
    #[serde(flatten)]
    pub base: Object,
}

impl Delete {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Delete".to_string()))
    }
}

impl StreamTrait for Delete {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Dislike {
    #[serde(flatten)]
    pub base: Object,
}

impl Dislike {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Dislike".to_string()))
    }
}

impl StreamTrait for Dislike {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Flag {
    #[serde(flatten)]
    pub base: Object,
}

impl Flag {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Flag".to_string()))
    }
}

impl StreamTrait for Flag {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Follow {
    #[serde(flatten)]
    pub base: Object,
}

impl Follow {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Follow".to_string()))
    }
}

impl StreamTrait for Follow {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Ignore {
    #[serde(flatten)]
    pub base: Object,
}

impl Ignore {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Ignore".to_string()))
    }
}

impl StreamTrait for Ignore {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Invite {
    #[serde(flatten)]
    pub base: Object,
}

impl Invite {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Invite".to_string()))
    }
}

impl StreamTrait for Invite {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Join {
    #[serde(flatten)]
    pub base: Object,
}

impl Join {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Join".to_string()))
    }
}

impl StreamTrait for Join {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Leave {
    #[serde(flatten)]
    pub base: Object,
}

impl Leave {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Leave".to_string()))
    }
}

impl StreamTrait for Leave {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Like {
    #[serde(flatten)]
    pub base: Object,
}

impl Like {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Like".to_string()))
    }
}

impl StreamTrait for Like {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Listen {
    #[serde(flatten)]
    pub base: Object,
}

impl Listen {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Listen".to_string()))
    }
}

impl StreamTrait for Listen {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Move {
    #[serde(flatten)]
    pub base: Object,
}

impl Move {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Move".to_string()))
    }
}

impl StreamTrait for Move {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Offer {
    #[serde(flatten)]
    pub base: Object,
}

impl Offer {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Offer".to_string()))
    }
}

impl StreamTrait for Offer {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Question {
    #[serde(flatten)]
    pub base: Object,
}

impl Question {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Question".to_string()))
    }
}

impl StreamTrait for Question {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Read {
    #[serde(flatten)]
    pub base: Object,
}

impl Read {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Read".to_string()))
    }
}

impl StreamTrait for Read {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Reject {
    #[serde(flatten)]
    pub base: Object,
}

impl Reject {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Reject".to_string()))
    }
}

impl StreamTrait for Reject {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Remove {
    #[serde(flatten)]
    pub base: Object,
}

impl Remove {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Remove".to_string()))
    }
}

impl StreamTrait for Remove {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct TentativeAccept {
    #[serde(flatten)]
    pub base: Object,
}

impl TentativeAccept {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("TentativeAccept".to_string()))
    }
}

impl StreamTrait for TentativeAccept {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct TentativeReject {
    #[serde(flatten)]
    pub base: Object,
}

impl TentativeReject {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("TentativeReject".to_string()))
    }
}

impl StreamTrait for TentativeReject {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Travel {
    #[serde(flatten)]
    pub base: Object,
}

impl Travel {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Travel".to_string()))
    }
}

impl StreamTrait for Travel {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Undo {
    #[serde(flatten)]
    pub base: Object,
}

impl Undo {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Undo".to_string()))
    }
}

impl StreamTrait for Undo {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Update {
    #[serde(flatten)]
    pub base: Object,
}

impl Update {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Update".to_string()))
    }
}

impl StreamTrait for Update {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct View {
    #[serde(flatten)]
    pub base: Object,
}

impl View {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("View".to_string()))
    }
}

impl StreamTrait for View {
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
