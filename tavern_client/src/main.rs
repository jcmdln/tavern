use tavern::traits::{ObjectTrait, StreamTrait};
use tavern::{activity, actor, core, object, property};

fn main() {
    let thing = core::Object::default()
        .kind(property::Type::String("Thing".to_string()))
        .id("http://domain.tld".to_string())
        .to(property::To::String("someone".to_string()))
        .to_value()
        .unwrap();
    println!("{}", thing);

    println!("\nCore:");
    println!("{}", core::Object::default().to_value().unwrap());
    println!("{}", core::Link::default().to_value().unwrap());
    println!("{}", core::Activity::default().to_value().unwrap());
    println!("{}", core::IntransitiveActivity::default().to_value().unwrap());
    println!("{}", core::Collection::default().to_value().unwrap());
    println!("{}", core::OrderedCollection::default().to_value().unwrap());
    println!("{}", core::CollectionPage::default().to_value().unwrap());
    println!("{}", core::OrderedCollectionPage::default().to_value().unwrap());

    println!("\nActivity:");
    println!("{}", activity::Accept::default().to_value().unwrap());
    println!("{}", activity::Add::default().to_value().unwrap());
    println!("{}", activity::Announce::default().to_value().unwrap());
    println!("{}", activity::Arrive::default().to_value().unwrap());
    println!("{}", activity::Block::default().to_value().unwrap());
    println!("{}", activity::Create::default().to_value().unwrap());
    println!("{}", activity::Delete::default().to_value().unwrap());
    println!("{}", activity::Dislike::default().to_value().unwrap());
    println!("{}", activity::Flag::default().to_value().unwrap());
    println!("{}", activity::Follow::default().to_value().unwrap());
    println!("{}", activity::Ignore::default().to_value().unwrap());
    println!("{}", activity::Invite::default().to_value().unwrap());
    println!("{}", activity::Join::default().to_value().unwrap());
    println!("{}", activity::Leave::default().to_value().unwrap());
    println!("{}", activity::Like::default().to_value().unwrap());
    println!("{}", activity::Listen::default().to_value().unwrap());
    println!("{}", activity::Move::default().to_value().unwrap());
    println!("{}", activity::Offer::default().to_value().unwrap());
    println!("{}", activity::Question::default().to_value().unwrap());
    println!("{}", activity::Read::default().to_value().unwrap());
    println!("{}", activity::Reject::default().to_value().unwrap());
    println!("{}", activity::Remove::default().to_value().unwrap());
    println!("{}", activity::TentativeAccept::default().to_value().unwrap());
    println!("{}", activity::TentativeReject::default().to_value().unwrap());
    println!("{}", activity::Travel::default().to_value().unwrap());
    println!("{}", activity::Undo::default().to_value().unwrap());
    println!("{}", activity::Update::default().to_value().unwrap());
    println!("{}", activity::View::default().to_value().unwrap());

    println!("\nActor:");
    println!("{}", actor::Actor::default().to_value().unwrap());
    println!("{}", actor::Application::default().to_value().unwrap());
    println!("{}", actor::Group::default().to_value().unwrap());
    println!("{}", actor::Organization::default().to_value().unwrap());
    println!("{}", actor::Person::default().to_value().unwrap());
    println!("{}", actor::Service::default().to_value().unwrap());

    println!("\nObject:");
    println!("{}", object::Article::default().to_value().unwrap());
    println!("{}", object::Audio::default().to_value().unwrap());
    println!("{}", object::Document::default().to_value().unwrap());
    println!("{}", object::Event::default().to_value().unwrap());
    println!("{}", object::Image::default().to_value().unwrap());
    println!("{}", object::Mention::default().to_value().unwrap());
    println!("{}", object::Note::default().to_value().unwrap());
    println!("{}", object::Page::default().to_value().unwrap());
    println!("{}", object::Place::default().to_value().unwrap());
    println!("{}", object::Profile::default().to_value().unwrap());
    println!("{}", object::Relationship::default().to_value().unwrap());
    println!("{}", object::Tombstone::default().to_value().unwrap());
    println!("{}", object::Video::default().to_value().unwrap());
}
