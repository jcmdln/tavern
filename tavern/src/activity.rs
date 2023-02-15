use crate::{core::Object, impl_Object_for, property, traits::StreamTrait};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Accept {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Accept);

impl Accept {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Accept".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Add {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Add);

impl Add {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Add".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Announce {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Announce);

impl Announce {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Announce".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Arrive {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Arrive);

impl Arrive {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Arrive".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Block {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Block);

impl Block {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Block".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Create {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Create);

impl Create {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Create".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Delete {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Delete);

impl Delete {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Delete".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Dislike {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Dislike);

impl Dislike {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Dislike".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Flag {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Flag);

impl Flag {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Flag".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Follow {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Follow);

impl Follow {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Follow".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Ignore {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Ignore);

impl Ignore {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Ignore".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Invite {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Invite);

impl Invite {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Invite".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Join {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Join);

impl Join {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Join".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Leave {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Leave);

impl Leave {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Leave".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Like {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Like);

impl Like {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Like".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Listen {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Listen);

impl Listen {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Listen".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Move {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Move);

impl Move {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Move".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Offer {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Offer);

impl Offer {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Offer".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Question {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Question);

impl Question {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Question".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Read {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Read);

impl Read {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Read".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Reject {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Reject);

impl Reject {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Reject".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Remove {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Remove);

impl Remove {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Remove".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct TentativeAccept {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(TentativeAccept);

impl TentativeAccept {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("TentativeAccept".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct TentativeReject {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(TentativeReject);

impl TentativeReject {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("TentativeReject".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Travel {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Travel);

impl Travel {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Travel".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Undo {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Undo);

impl Undo {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Undo".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Update {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(Update);

impl Update {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("Update".to_string()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct View {
    #[serde(flatten)]
    pub base: Object,
}

impl_Object_for!(View);

impl View {
    pub fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String(
                "https://www.w3.org/ns/activitystreams".to_string(),
            ))
            .r#type(property::Type::String("View".to_string()))
    }
}
