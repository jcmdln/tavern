use serde_json::json;

use crate::{core::Object, traits::StreamTrait};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Accept(Object);

impl Accept {
    pub fn new() -> Object {
        Object::new().r#type(json!("Accept"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Add(Object);

impl Add {
    pub fn new() -> Object {
        Object::new().r#type(json!("Add"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Announce(Object);

impl Announce {
    pub fn new() -> Object {
        Object::new().r#type(json!("Announce"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Arrive(Object);

impl Arrive {
    pub fn new() -> Object {
        Object::new().r#type(json!("Arrive"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Block(Object);

impl Block {
    pub fn new() -> Object {
        Object::new().r#type(json!("Block"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Create(Object);

impl Create {
    pub fn new() -> Object {
        Object::new().r#type(json!("Create"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Delete(Object);

impl Delete {
    pub fn new() -> Object {
        Object::new().r#type(json!("Delete"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Dislike(Object);

impl Dislike {
    pub fn new() -> Object {
        Object::new().r#type(json!("Dislike"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Flag(Object);

impl Flag {
    pub fn new() -> Object {
        Object::new().r#type(json!("Flag"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Follow(Object);

impl Follow {
    pub fn new() -> Object {
        Object::new().r#type(json!("Follow"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Ignore(Object);

impl Ignore {
    pub fn new() -> Object {
        Object::new().r#type(json!("Ignore"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Invite(Object);

impl Invite {
    pub fn new() -> Object {
        Object::new().r#type(json!("Invite"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Join(Object);

impl Join {
    pub fn new() -> Object {
        Object::new().r#type(json!("Join"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Leave(Object);

impl Leave {
    pub fn new() -> Object {
        Object::new().r#type(json!("Leave"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Like(Object);

impl Like {
    pub fn new() -> Object {
        Object::new().r#type(json!("Like"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Listen(Object);

impl Listen {
    pub fn new() -> Object {
        Object::new().r#type(json!("Listen"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Move(Object);

impl Move {
    pub fn new() -> Object {
        Object::new().r#type(json!("Move"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Offer(Object);

impl Offer {
    pub fn new() -> Object {
        Object::new().r#type(json!("Offer"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Question(Object);

impl Question {
    pub fn new() -> Object {
        Object::new().r#type(json!("Question"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Read(Object);

impl Read {
    pub fn new() -> Object {
        Object::new().r#type(json!("Read"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Reject(Object);

impl Reject {
    pub fn new() -> Object {
        Object::new().r#type(json!("Reject"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Remove(Object);

impl Remove {
    pub fn new() -> Object {
        Object::new().r#type(json!("Remove"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct TentativeAccept(Object);

impl TentativeAccept {
    pub fn new() -> Object {
        Object::new().r#type(json!("TentativeAccept"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct TentativeReject(Object);

impl TentativeReject {
    pub fn new() -> Object {
        Object::new().r#type(json!("TentativeReject"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Travel(Object);

impl Travel {
    pub fn new() -> Object {
        Object::new().r#type(json!("Travel"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Undo(Object);

impl Undo {
    pub fn new() -> Object {
        Object::new().r#type(json!("Undo"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Update(Object);

impl Update {
    pub fn new() -> Object {
        Object::new().r#type(json!("Update"))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct View(Object);

impl View {
    pub fn new() -> Object {
        Object::new().r#type(json!("View"))
    }
}
