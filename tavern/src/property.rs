use crate::{core, object};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Describes one or more entities that either performed or are expected to
/// perform the activity. Any single activity can have multiple actors. The
/// actor may be specified using an indirect Link.
///
/// URI: https://www.w3.org/ns/activitystreams#actor
///
/// Domain: `Object`
///
/// Subproperty Of: `attributed_to`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Actor {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<Actor>),
}

/// Identifies an inclusive option for a Question. Use of anyOf implies that
/// the Question can have multiple answers. To indicate that a Question can
/// have only one answer, use oneOf.
///
/// URI: https://www.w3.org/ns/activitystreams#anyOf
///
/// Domain: `Question`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnyOf {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<AnyOf>),
}

/// Identifies the processing context by reference to its normative definition.
/// Implementations may augment the provided @context with additional @context
/// definitions but must not override or change the normative context.
///
/// URI: @context
///
/// Domain: `Object` | `Link`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AtContext {
    Map(HashMap<String, String>),
    String(String),
    Vec(Vec<AtContext>),
}

/// Identifies a resource attached or related to an object that potentially
/// requires special handling. The intent is to provide a model that is at
/// least semantically similar to attachments in email.
///
/// URI: https://www.w3.org/ns/activitystreams#attachment
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Attachment {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<Attachment>),
}

/// Identifies one or more entities to which this object is attributed. The
/// attributed entities might not be Actors. For instance, an object might be
/// attributed to the completion of another activity.
///
/// URI: https://www.w3.org/ns/activitystreams#attributed_to
///
/// Domain: `Link` | `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AttributedTo {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<AttributedTo>),
}

/// Identifies one or more entities that represent the total population of
/// entities for which the object can considered to be relevant.
///
/// URI: https://www.w3.org/ns/activitystreams#audience
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Audience {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<Audience>),
}

/// Identifies one or more Objects that are part of the private secondary
/// audience of this Object.
///
/// URI: https://www.w3.org/ns/activitystreams#bcc
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Bcc {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<Bcc>),
}

/// Identifies an `Object` that is part of the private primary audience of this
/// Object.
///
/// URI: https://www.w3.org/ns/activitystreams#bto
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Bto {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<Bto>),
}

/// Identifies an Object that is part of the public secondary audience of this
/// Object.
///
/// URI: https://www.w3.org/ns/activitystreams#cc
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Cc {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<Cc>),
}

/// Indicates that a question has been closed, and answers are no longer
/// accepted.
///
/// URI: https://www.w3.org/ns/activitystreams#closed
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Closed {
    Boolean(bool),
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
}

/// Identifies the context within which the object exists or an activity was
/// performed.
///
/// The notion of "context" used is intentionally vague. The intended function
/// is to serve as a means of grouping objects and activities that share a
/// common originating context or purpose. An example could be all activities
/// relating to a common project or event.
///
/// URI: https://www.w3.org/ns/activitystreams#context
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Context {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<Context>),
}

/// In a paged Collection, indicates the page that contains the most recently
/// updated member items.
///
/// URI: https://www.w3.org/ns/activitystreams#current
///
/// Domain: Collection
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Current {
    CollectionPage(core::CollectionPage),
    Link(core::Link),
    String(String),
}

/// On a Profile object, the describes property identifies the object described
/// by the Profile.
///
/// URI: https://www.w3.org/ns/activitystreams#describes
///
/// Domain: `Profile`
///
/// Functional: `True`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Describes {
    Object(core::Object),
    String(String),
}

/// In a paged Collection, indicates the furthest preceeding page of items in
/// the collection.
///
/// URI: https://www.w3.org/ns/activitystreams#first
///
/// Domain: `Collection`
///
/// Functional: `True`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum First {
    CollectionPage(core::CollectionPage),
    Link(core::Link),
    String(String),
}

/// On a Profile object, the describes property identifies the object described
/// by the Profile.
///
/// URI: https://www.w3.org/ns/activitystreams#formerType
///
/// Domain: `Profile`
///
/// Functional: `False`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FormerType {
    Object(core::Object),
    String(String),
}

/// Identifies the entity (e.g. an application) that generated the object.
///
/// URI: https://www.w3.org/ns/activitystreams#generator
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Generator {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<Generator>),
}

/// Indicates an entity that describes an icon for this object. The image
/// should have an aspect ratio of one (horizontal) to one (vertical) and
/// should be suitable for presentation at a small size.
///
/// URI: https://www.w3.org/ns/activitystreams#icon
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Icon {
    Image(Box<object::Image>),
    Link(core::Link),
    Vec(Vec<Icon>),
}

/// Indicates an entity that describes an image for this object. Unlike the
/// icon property, there are no aspect ratio or display size limitations
/// assumed.
///
/// URI: https://www.w3.org/ns/activitystreams#image
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Image {
    Image(Box<object::Image>),
    Link(core::Link),
    Vec(Vec<Image>),
}

/// Indicates one or more entities for which this object is considered a
/// response.
///
/// URI: https://www.w3.org/ns/activitystreams#in_reply_to
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InReplyTo {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<InReplyTo>),
}

/// Identifies one or more objects used (or to be used) in the completion of an
/// Activity.
///
/// URI: https://www.w3.org/ns/activitystreams#instrument
///
/// Domain: `Activity`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Instrument {
    String(String),
}

/// Identifies the items contained in a collection. The items might be ordered
/// or unordered.
///
/// URI: https://www.w3.org/ns/activitystreams#items
///
/// Domain: `Collection`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Items {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<Items>),
}

/// In a paged Collection, indicates the furthest proceeding page of the
/// collection.
///
/// URI: https://www.w3.org/ns/activitystreams#last
///
/// Domain: `Collection`
///
/// Functional: `True`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Last {
    String(String),
}

/// Indicates one or more physical or logical locations associated with the
/// object.
///
/// URI: https://www.w3.org/ns/activitystreams#location
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Location {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<Location>),
}

/// In a paged Collection, indicates the next page of items.
///
/// URI: https://www.w3.org/ns/activitystreams#next
///
/// Domain: `CollectionPage`
///
/// Functional: `True`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Next {
    CollectionPage(Box<core::CollectionPage>),
    Link(core::Link),
    String(String),
}

/// When used within an Activity, describes the direct object of the activity.
/// For instance, in the activity "John added a movie to his wishlist", the
/// object of the activity is the movie added.
///
/// When used within a Relationship describes the entity to which the subject
/// is related.
///
/// URI: https://www.w3.org/ns/activitystreams#object
///
/// Domain: `Activity` | `Relationship`
///
/// Functional: `True`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Object {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<Object>),
}

/// Identifies an exclusive option for a Question. Use of oneOf implies that
/// the Question can have only a single answer. To indicate that a Question can
/// have multiple answers, use anyOf.
///
/// URI: https://www.w3.org/ns/activitystreams#oneOf
///
/// Domain: `Question`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOf {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<OneOf>),
}

/// Describes an indirect object of the activity from which the activity is
/// directed. The precise meaning of the origin is the object of the English
/// preposition "from". For instance, in the activity "John moved an item to
/// List B from List A", the origin of the activity is "List A".
///
/// URI: https://www.w3.org/ns/activitystreams#origin
///
/// Domain: `Activity`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Origin {
    Link(core::Link),
    Object(Box<core::Object>),
}

/// Identifies the Collection to which a CollectionPage objects items belong.
///
/// URI: https://www.w3.org/ns/activitystreams#partOf
///
/// Domain: `CollectionPage`
///
/// Functional: `True`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PartOf {
    Collection(Box<core::Collection>),
    Link(core::Link),
    String(String),
}

/// In a paged Collection, indicates the previous page of items.
///
/// URI: https://www.w3.org/ns/activitystreams#prev
///
/// Domain: `CollectionPage`
///
/// Functional: `True`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Prev {
    CollectionPage(Box<core::CollectionPage>),
    Link(core::Link),
    String(String),
}

/// Identifies an entity that provides a preview of this object.
///
/// URI: https://www.w3.org/ns/activitystreams#preview
///
/// Domain: `Link` | `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Preview {
    Link(Box<core::Link>),
    Object(Box<core::Object>),
}

/// A link relation associated with a Link. The value must conform to both the
/// [HTML5] and [RFC5988] "link relation" definitions.
///
/// In the [HTML5], any string not containing the "space" U+0020, "tab"
/// (U+0009), "LF" (U+000A), "FF" (U+000C), "CR" (U+000D) or "," (U+002C)
/// characters can be used as a valid link relation.
///
/// URI: https://www.w3.org/ns/activitystreams#rel
///
/// Domain: `Link`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Rel {
    String(String),
    Vec(Vec<String>),
}

/// On a Relationship object, the relationship property identifies the kind of
/// relationship that exists between subject and object.
///
/// URI: https://www.w3.org/ns/activitystreams#relationship
///
/// Domain: `Relationship`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Relationship {
    Object(core::Object),
    String(String),
}

/// Identifies a Collection containing objects considered to be responses to
/// this object.
///
/// URI: https://www.w3.org/ns/activitystreams#replies
///
/// Domain: `Object`
///
/// Functional: `True`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Replies {
    Collection(Box<core::Collection>),
}

/// Describes the result of the activity. For instance, if a particular action
/// results in the creation of a new resource, the result property can be used
/// to describe that new resource.
///
/// URI: https://www.w3.org/ns/activitystreams#result
///
/// Domain: `Activity`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Result {
    Link(core::Link),
    Object(core::Object),
}

/// On a Relationship object, the subject property identifies one of the
/// connected individuals. For instance, for a Relationship object describing
/// "John is related to Sally", subject would refer to John.
///
/// URI: https://www.w3.org/ns/activitystreams#subject
///
/// Domain: `Relationship`
///
/// Functional: `True`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Subject {
    Object(core::Object),
    String(String),
}

/// One or more "tags" that have been associated with an objects. A tag can be
/// any kind of Object. The key difference between attachment and tag is that
/// the former implies association by inclusion, while the latter implies
/// associated by reference.
///
/// URI: https://www.w3.org/ns/activitystreams#tag
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Tag {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<Tag>),
}

/// Describes the indirect object, or target, of the activity. The precise
/// meaning of the target is largely dependent on the type of action being
/// described but will often be the object of the English preposition "to". For
/// instance, in the activity "John added a movie to his wishlist", the target
/// of the activity is John's wishlist. An activity can have more than one
/// target.
///
/// URI: https://www.w3.org/ns/activitystreams#target
///
/// Domain: `Activity`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Target {
    Link(core::Link),
    Object(core::Object),
    String(String),
    Vec(Vec<Target>),
}

/// Identifies an entity considered to be part of the public primary audience
/// of an Object.
///
/// URI: https://www.w3.org/ns/activitystreams#to
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum To {
    Link(core::Link),
    Object(Box<core::Object>),
    String(String),
    Vec(Vec<Target>),
}

/// Identifies the Object or Link type. Multiple values may be specified.
///
/// URI: @type
///
/// Domain: `Object` | `Link`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Type {
    String(String),
    Vec(Vec<String>),
}

/// Identifies an entity considered to be part of the public primary audience
/// of an Object.
///
/// URI: https://www.w3.org/ns/activitystreams#url
///
/// Domain: `Object`
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Url {
    Link(core::Link),
    String(String),
    Vec(Vec<Url>),
}
