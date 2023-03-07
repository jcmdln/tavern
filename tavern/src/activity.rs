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
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Accept {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` has added the `object` to the `target`. If the
/// `target` property is not explicitly specified, the target would need to be
/// determined implicitly by context. The `origin` can be used to identify the
/// context from which the `object` originated.
///
/// URI: https://www.w3.org/ns/activitystreams#Add
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Add {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` is calling the `target`'s attention to the
/// `object`.
///
/// The `origin` typically has no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Announce
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Announce {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// An `IntransitiveActivity` that indicates that the `actor` has arrived at
/// the `location`. The `origin` can be used to identify the context from which
/// the `actor` originated. The `target` typically has no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Arrive
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Arrive {
    #[serde(flatten)]
    pub extends: core::IntransitiveActivity,
}

/// Indicates that the `actor` is blocking the `object`. Blocking is a stronger
/// form of `Ignore`. The typical use is to support social systems that allow
/// one user to block activities or content of other users. The `target` and
/// `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Block
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` has created the `object`.
///
/// URI: https://www.w3.org/ns/activitystreams#Create
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Create {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` has deleted the `object`. If specified, the
/// `origin` indicates the context from which the `object` was deleted.
///
/// URI: https://www.w3.org/ns/activitystreams#Delete
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Delete {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` dislikes the `object`.
///
/// URI: https://www.w3.org/ns/activitystreams#Dislike
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dislike {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` is "flagging" the `object`. Flagging is defined
/// in the sense common to many social platforms as reporting content as being
/// inappropriate for any number of reasons.
///
/// URI: https://www.w3.org/ns/activitystreams#Flag
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Flag {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` is "following" the `object`. Following is
/// defined in the sense typically used within Social systems in which the
/// actor is interested in any activity performed by or on the object. The
/// `target` and `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Follow
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Follow {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` is ignoring the `object`. The `target` and
/// `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Ignore
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ignore {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// A specialization of `Offer` in which the `actor` is extending an invitation
/// for the `object` to the `target`.
///
/// URI: https://www.w3.org/ns/activitystreams#Invite
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Invite {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` has joined the `object`. The `target` and
/// `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Join
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Join {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` has left the `object`. The `target` and `origin`
/// typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Leave
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Leave {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` likes, recommends or endorses the `object`. The
/// `target` and `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Like
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Like {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` has listened to the `object`.
///
/// URI: https://www.w3.org/ns/activitystreams#Listen
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Listen {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` has moved `object` from `origin` to `target`. If
/// the `origin` or `target` are not specified, either can be determined by
/// context.
///
/// URI: https://www.w3.org/ns/activitystreams#Move
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Move {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` is offering the `object`. If specified, the
/// `target` indicates the entity to which the `object` is being offered.
///
/// URI: https://www.w3.org/ns/activitystreams#Offer
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Offer {
    #[serde(flatten)]
    pub extends: core::Activity,
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
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    #[serde(flatten)]
    pub extends: core::IntransitiveActivity,

    pub any_of: Option<property::AnyOf>,
    pub closed: Option<property::Closed>,
    pub one_of: Option<property::OneOf>,
}

impl Default for Question {
    fn default() -> Self {
        Self {
            extends: core::IntransitiveActivity::default()
                .kind(property::Type::String(stringify!(Question).into()))
                .clone(),
            any_of: None,
            closed: None,
            one_of: None,
        }
    }
}

impl StreamTrait for Question {
    fn as_stream(&mut self) -> &mut core::Stream {
        &mut self.as_object().extends
    }
}

impl ObjectTrait for Question {
    fn as_object(&mut self) -> &mut core::Object {
        &mut self.as_intransitive_activity().extends
    }
}

impl IntransitiveActivityTrait for Question {
    fn as_intransitive_activity(&mut self) -> &mut core::IntransitiveActivity {
        &mut self.extends
    }
}

/// Indicates that the `actor` has read the `object`.
///
/// URI: https://www.w3.org/ns/activitystreams#Read
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Read {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` is rejecting the `object`. The `target` and
/// `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Reject
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reject {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` is removing the `object`. If specified, the
/// `origin` indicates the context from which the `object` is being removed.
///
/// URI: https://www.w3.org/ns/activitystreams#Remove
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Remove {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// A specialization of `Accept` indicating that the acceptance is tentative.
///
/// URI: https://www.w3.org/ns/activitystreams#TentativeAccept
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TentativeAccept {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// A specialization of `Reject` indicating that the rejection is tentative.
///
/// URI: https://www.w3.org/ns/activitystreams#TentativeReject
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TentativeReject {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` is traveling to `target` from `origin`. `Travel`
/// is an `IntransitiveObject` whose `actor` specifies the direct object. If
/// the `origin` or `target` are not specified, either can be determined by
/// context.
///
/// URI: https://www.w3.org/ns/activitystreams#Travel
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Travel {
    #[serde(flatten)]
    pub extends: core::IntransitiveActivity,
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
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Undo {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` has updated the `object`. Note, however, that
/// this vocabulary does not define a mechanism for describing the actual set
/// of modifications made to `object`.
///
/// The `target` and `origin` typically have no defined meaning.
///
/// URI: https://www.w3.org/ns/activitystreams#Update
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Update {
    #[serde(flatten)]
    pub extends: core::Activity,
}

/// Indicates that the `actor` has viewed the `object`.
///
/// URI: https://www.w3.org/ns/activitystreams#View
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct View {
    #[serde(flatten)]
    pub extends: core::Activity,
}

macro_rules! activity_impl {
    ($($t:ty)*) => ($(
        impl Default for $t {
            fn default() -> Self {
                Self {
                    extends: $crate::core::Activity::default()
                        .kind($crate::property::Type::String(stringify!($t).into()))
                        .clone(),
                }
            }
        }

        impl $crate::traits::StreamTrait for $t {
            fn as_stream(&mut self) -> &mut $crate::core::Stream {
                &mut self.as_object().extends
            }
        }

        impl $crate::traits::ObjectTrait for $t {
            fn as_object(&mut self) -> &mut $crate::core::Object {
                &mut self.as_activity().extends
            }
        }

        impl $crate::traits::ActivityTrait for $t {
            fn as_activity(&mut self) -> &mut $crate::core::Activity {
                &mut self.extends
            }
        }
    )*)
}

activity_impl! {
    Accept Add Announce Block Create Delete Dislike Flag Follow Ignore Invite Join Leave Like
    Listen Move Offer Read Reject Remove TentativeAccept TentativeReject Undo Update View
}

macro_rules! intransitive_activity {
    ($($t:ty)*) => ($(
        impl Default for $t {
            fn default() -> Self {
                Self {
                    extends: $crate::core::IntransitiveActivity::default()
                        .kind($crate::property::Type::String(stringify!($t).into()))
                        .clone(),
                }
            }
        }

        impl $crate::traits::StreamTrait for $t {
            fn as_stream(&mut self) -> &mut $crate::core::Stream {
                &mut self.as_object().extends
            }
        }

        impl $crate::traits::ObjectTrait for $t {
            fn as_object(&mut self) -> &mut $crate::core::Object {
                &mut self.as_intransitive_activity().extends
            }
        }

        impl $crate::traits::IntransitiveActivityTrait for $t {
            fn as_intransitive_activity(&mut self) -> &mut $crate::core::IntransitiveActivity {
                &mut self.extends
            }
        }
    )*)
}

intransitive_activity! { Arrive Travel }
