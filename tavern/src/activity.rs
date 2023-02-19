use crate::{
    core, property,
    traits::{ActivityTrait, IntransitiveActivityTrait, ObjectTrait, StreamTrait},
};
use serde::{Deserialize, Serialize};

/// Indicates that the `actor` accepts the `object`. The `target` property can
/// be used in certain circumstances to indicate the context into which the
/// `object` has been accepted.
///
/// URI: https://www.w3.org/ns/activitystreams#Accept
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Accept {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Accept {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Accept".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Accept {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Accept {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` has added the `object` to the `target`. If the
/// `target` property is not explicitly specified, the target would need to be
/// determined implicitly by context. The `origin` can be used to identify the
/// context from which the `object` originated.
///
/// URI: https://www.w3.org/ns/activitystreams#Add
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Add {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Add {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Add".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Add {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Add {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` is calling the `target`'s attention to the
/// `object`.
///
/// The `origin` typically has no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Announce
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Announce {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Announce {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Announce".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Announce {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Announce {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// An `IntransitiveActivity` that indicates that the `actor` has arrived at
/// the `location`. The `origin` can be used to identify the context from which
/// the `actor` originated. The `target` typically has no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Arrive
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Arrive {
    #[serde(flatten)]
    pub extends: core::IntransitiveActivity,
}

impl StreamTrait for Arrive {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Arrive".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Arrive {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_intransitive_activity().extends
    }
}

impl IntransitiveActivityTrait for Arrive {
    fn as_intransitive_activity(&mut self) -> &mut crate::core::IntransitiveActivity {
        &mut self.extends
    }
}

/// Indicates that the `actor` is blocking the `object`. Blocking is a stronger
/// form of `Ignore`. The typical use is to support social systems that allow
/// one user to block activities or content of other users. The `target` and
/// `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Block
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Block {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Block {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Block".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Block {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Block {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` has created the `object`.
///
/// URI: https://www.w3.org/ns/activitystreams#Create
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Create {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Create {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Create".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Create {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Create {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` has deleted the `object`. If specified, the
/// `origin` indicates the context from which the `object` was deleted.
///
/// URI: https://www.w3.org/ns/activitystreams#Delete
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Delete {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Delete {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Delete".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Delete {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Delete {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` dislikes the `object`.
///
/// URI: https://www.w3.org/ns/activitystreams#Dislike
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Dislike {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Dislike {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Dislike".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Dislike {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Dislike {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` is "flagging" the `object`. Flagging is defined
/// in the sense common to many social platforms as reporting content as being
/// inappropriate for any number of reasons.
///
/// URI: https://www.w3.org/ns/activitystreams#Flag
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Flag {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Flag {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Flag".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Flag {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Flag {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` is "following" the `object`. Following is
/// defined in the sense typically used within Social systems in which the
/// actor is interested in any activity performed by or on the object. The
/// `target` and `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Follow
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Follow {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Follow {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Follow".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Follow {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Follow {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` is ignoring the `object`. The `target` and
/// `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Ignore
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Ignore {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Ignore {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Ignore".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Ignore {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Ignore {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// A specialization of `Offer` in which the `actor` is extending an invitation
/// for the `object` to the `target`.
///
/// URI: https://www.w3.org/ns/activitystreams#Invite
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Invite {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Invite {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Invite".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Invite {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Invite {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` has joined the `object`. The `target` and
/// `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Join
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Join {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Join {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Join".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Join {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Join {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` has left the `object`. The `target` and `origin`
/// typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Leave
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Leave {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Leave {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Leave".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Leave {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Leave {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` likes, recommends or endorses the `object`. The
/// `target` and `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Like
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Like {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Like {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Like".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Like {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Like {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` has listened to the `object`.
///
/// URI: https://www.w3.org/ns/activitystreams#Listen
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Listen {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Listen {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Listen".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Listen {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Listen {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` has moved `object` from `origin` to `target`. If
/// the `origin` or `target` are not specified, either can be determined by
/// context.
///
/// URI: https://www.w3.org/ns/activitystreams#Move
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Move {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Move {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Move".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Move {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Move {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` is offering the `object`. If specified, the
/// `target` indicates the entity to which the `object` is being offered.
///
/// URI: https://www.w3.org/ns/activitystreams#Offer
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Offer {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Offer {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Offer".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Offer {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Offer {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Represents a question being asked. Question objects are an extension of
/// `IntransitiveActivity`. That is, the Question object is an Activity, but
/// the direct object is the question itself and therefore it would not contain
/// an `object` property.
///
/// Either of the `anyOf` and `oneOf` properties **may** be used to express
/// possible answers, but a Question object **must not** have both properties.
///
/// URI: https://www.w3.org/ns/activitystreams#Question
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Question {
    #[serde(flatten)]
    pub extends: core::IntransitiveActivity,

    pub anyOf: Option<property::AnyOf>,
    pub closed: Option<property::Closed>,
    pub oneOf: Option<property::OneOf>,
}

impl StreamTrait for Question {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Question".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Question {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_intransitive_activity().extends
    }
}

impl IntransitiveActivityTrait for Question {
    fn as_intransitive_activity(&mut self) -> &mut crate::core::IntransitiveActivity {
        &mut self.extends
    }
}

/// Indicates that the `actor` has read the `object`.
///
/// URI: https://www.w3.org/ns/activitystreams#Read
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Read {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Read {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Read".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Read {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Read {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` is rejecting the `object`. The `target` and
/// `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Reject
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Reject {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Reject {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Reject".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Reject {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Reject {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` is removing the `object`. If specified, the
/// `origin` indicates the context from which the `object` is being removed.
///
/// URI: https://www.w3.org/ns/activitystreams#Remove
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Remove {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Remove {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Remove".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Remove {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Remove {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// A specialization of `Accept` indicating that the acceptance is tentative.
///
/// URI: https://www.w3.org/ns/activitystreams#TentativeAccept
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TentativeAccept {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for TentativeAccept {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("TentativeAccept".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for TentativeAccept {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for TentativeAccept {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// A specialization of `Reject` indicating that the rejection is tentative.
///
/// URI: https://www.w3.org/ns/activitystreams#TentativeReject
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TentativeReject {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for TentativeReject {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("TentativeReject".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for TentativeReject {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for TentativeReject {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` is traveling to `target` from `origin`. `Travel`
/// is an `IntransitiveObject` whose `actor` specifies the direct object. If
/// the `origin` or `target` are not specified, either can be determined by
/// context.
///
/// URI: https://www.w3.org/ns/activitystreams#Travel
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Travel {
    #[serde(flatten)]
    pub extends: core::IntransitiveActivity,
}

impl StreamTrait for Travel {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Travel".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Travel {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_intransitive_activity().extends
    }
}

impl IntransitiveActivityTrait for Travel {
    fn as_intransitive_activity(&mut self) -> &mut crate::core::IntransitiveActivity {
        &mut self.extends
    }
}

/// Indicates that the `actor` is undoing the `object`. In most cases, the
/// `object` will be an `Activity` describing some previously performed action
/// (for instance, a person may have previously "liked" an article but, for
/// whatever reason, might choose to undo that like at some later point in
/// time).
///
/// The `target` and `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Undo
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Undo {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Undo {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Undo".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Undo {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Undo {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` has updated the `object`. Note, however, that
/// this vocabulary does not define a mechanism for describing the actual set
/// of modifications made to `object`.
///
/// The `target` and `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Update
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Update {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for Update {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("Update".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Update {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for Update {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}

/// Indicates that the `actor` has viewed the `object`.
///
/// URI: https://www.w3.org/ns/activitystreams#View
#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct View {
    #[serde(flatten)]
    pub extends: core::Activity,
}

impl StreamTrait for View {
    fn new() -> Self {
        Self::default()
            .atContext(property::AtContext::String("https://www.w3.org/ns/activitystreams".into()))
            .r#type(property::Type::String("View".into()))
            .to_owned()
    }

    fn as_stream(&mut self) -> &mut crate::core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for View {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_activity().extends
    }
}

impl ActivityTrait for View {
    fn as_activity(&mut self) -> &mut crate::core::Activity {
        &mut self.extends
    }
}
