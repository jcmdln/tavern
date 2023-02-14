// use std::error::Error;

// use serde_json::Value;

pub mod activity;
pub mod actor;
pub mod core;
pub mod object;
pub mod property;
pub mod traits;

pub enum Stream {
    Object(core::Object),
    Link(core::Link),
    Activity(core::Activity),
    IntransitiveActivity(core::IntransitiveActivity),
    Collection(core::Collection),
    OrderedCollection(core::OrderedCollection),
    CollectionPage(core::CollectionPage),
    OrderedCollectionPage(core::OrderedCollectionPage),

    Accept(activity::Accept),
    Add(activity::Add),
    Announce(activity::Announce),
    Arrive(activity::Arrive),
    Block(activity::Block),
    Create(activity::Create),
    Delete(activity::Delete),
    Dislike(activity::Dislike),
    Flag(activity::Flag),
    Follow(activity::Follow),
    Ignore(activity::Ignore),
    Invite(activity::Invite),
    Join(activity::Join),
    Leave(activity::Leave),
    Like(activity::Like),
    Listen(activity::Listen),
    Move(activity::Move),
    Offer(activity::Offer),
    Question(activity::Question),
    Read(activity::Read),
    Reject(activity::Reject),
    Remove(activity::Remove),
    TentativeAccept(activity::TentativeAccept),
    TentativeReject(activity::TentativeReject),
    Travel(activity::Travel),
    Undo(activity::Undo),
    Update(activity::Update),
    View(activity::View),

    Actor(actor::Actor),
    Application(actor::Application),
    Group(actor::Group),
    Organization(actor::Organization),
    Person(actor::Person),
    Service(actor::Service),

    Article(object::Article),
    Audio(object::Audio),
    Document(object::Document),
    Event(object::Event),
    Image(object::Image),
    Mention(object::Mention),
    Note(object::Note),
    Page(object::Page),
    Place(object::Place),
    Profile(object::Profile),
    Relationship(object::Relationship),
    Tombstone(object::Tombstone),
    Video(object::Video),
}

impl Stream {
    // pub fn from_value(self, value: Value) -> Result<Self, Error> {
    //     todo!()
    // }
    // pub fn to_value(self) -> Result<Value, Error> {
    //     todo!()
    // }
}
