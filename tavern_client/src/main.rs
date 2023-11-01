// Clippy
#![warn(clippy::all, clippy::cargo, clippy::pedantic)]
#![allow(clippy::cargo_common_metadata)]
// Documentation
#![allow(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::undocumented_unsafe_blocks
)]
// Restrictions
#![warn(
    clippy::as_conversions,
    clippy::dbg_macro,
    clippy::empty_structs_with_brackets,
    clippy::from_over_into,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::let_underscore_must_use,
    clippy::map_err_ignore,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_used
)]

use tavern::traits::{ObjectTrait, StreamTrait};
use tavern::{activity, actor, core, object, property};

fn main() {
    let thing = core::Object::default()
        .kind(property::Type::String("Thing".into()))
        .id("http://domain.tld".into())
        .to(property::To::String("foo@bar.tld".into()))
        .bto(property::Bto::String("hello@world.tld".into()))
        .to_value()
        .unwrap_or_default();
    println!("{}", thing);

    println!("\nCore:");
    println!("{}", core::Object::default().to_value().unwrap_or_default());
    println!("{}", core::Link::default().to_value().unwrap_or_default());
    println!("{}", core::Activity::default().to_value().unwrap_or_default());
    println!("{}", core::IntransitiveActivity::default().to_value().unwrap_or_default());
    println!("{}", core::Collection::default().to_value().unwrap_or_default());
    println!("{}", core::OrderedCollection::default().to_value().unwrap_or_default());
    println!("{}", core::CollectionPage::default().to_value().unwrap_or_default());
    println!("{}", core::OrderedCollectionPage::default().to_value().unwrap_or_default());

    println!("\nActivity:");
    println!("{}", activity::Accept::default().to_value().unwrap_or_default());
    println!("{}", activity::Add::default().to_value().unwrap_or_default());
    println!("{}", activity::Announce::default().to_value().unwrap_or_default());
    println!("{}", activity::Arrive::default().to_value().unwrap_or_default());
    println!("{}", activity::Block::default().to_value().unwrap_or_default());
    println!("{}", activity::Create::default().to_value().unwrap_or_default());
    println!("{}", activity::Delete::default().to_value().unwrap_or_default());
    println!("{}", activity::Dislike::default().to_value().unwrap_or_default());
    println!("{}", activity::Flag::default().to_value().unwrap_or_default());
    println!("{}", activity::Follow::default().to_value().unwrap_or_default());
    println!("{}", activity::Ignore::default().to_value().unwrap_or_default());
    println!("{}", activity::Invite::default().to_value().unwrap_or_default());
    println!("{}", activity::Join::default().to_value().unwrap_or_default());
    println!("{}", activity::Leave::default().to_value().unwrap_or_default());
    println!("{}", activity::Like::default().to_value().unwrap_or_default());
    println!("{}", activity::Listen::default().to_value().unwrap_or_default());
    println!("{}", activity::Move::default().to_value().unwrap_or_default());
    println!("{}", activity::Offer::default().to_value().unwrap_or_default());
    println!("{}", activity::Question::default().to_value().unwrap_or_default());
    println!("{}", activity::Read::default().to_value().unwrap_or_default());
    println!("{}", activity::Reject::default().to_value().unwrap_or_default());
    println!("{}", activity::Remove::default().to_value().unwrap_or_default());
    println!("{}", activity::TentativeAccept::default().to_value().unwrap_or_default());
    println!("{}", activity::TentativeReject::default().to_value().unwrap_or_default());
    println!("{}", activity::Travel::default().to_value().unwrap_or_default());
    println!("{}", activity::Undo::default().to_value().unwrap_or_default());
    println!("{}", activity::Update::default().to_value().unwrap_or_default());
    println!("{}", activity::View::default().to_value().unwrap_or_default());

    println!("\nActor:");
    println!("{}", actor::Actor::default().to_value().unwrap_or_default());
    println!("{}", actor::Application::default().to_value().unwrap_or_default());
    println!("{}", actor::Group::default().to_value().unwrap_or_default());
    println!("{}", actor::Organization::default().to_value().unwrap_or_default());
    println!("{}", actor::Person::default().to_value().unwrap_or_default());
    println!("{}", actor::Service::default().to_value().unwrap_or_default());

    println!("\nObject:");
    println!("{}", object::Article::default().to_value().unwrap_or_default());
    println!("{}", object::Audio::default().to_value().unwrap_or_default());
    println!("{}", object::Document::default().to_value().unwrap_or_default());
    println!("{}", object::Event::default().to_value().unwrap_or_default());
    println!("{}", object::Image::default().to_value().unwrap_or_default());
    println!("{}", object::Mention::default().to_value().unwrap_or_default());
    println!("{}", object::Note::default().to_value().unwrap_or_default());
    println!("{}", object::Page::default().to_value().unwrap_or_default());
    println!("{}", object::Place::default().to_value().unwrap_or_default());
    println!("{}", object::Profile::default().to_value().unwrap_or_default());
    println!("{}", object::Relationship::default().to_value().unwrap_or_default());
    println!("{}", object::Tombstone::default().to_value().unwrap_or_default());
    println!("{}", object::Video::default().to_value().unwrap_or_default());
}
