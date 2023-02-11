use tavern::traits::{ObjectTrait, StreamTrait};
use tavern::{activity, actor, core, object, property};

fn main() {
    let thing = core::Object::new()
        .r#type(property::Type::String("Thing".to_string()))
        .id("http://domain.tld".to_string())
        .to(property::To::String("someone".to_string()))
        .to_string()
        .unwrap();
    println!("{}", thing);

    println!("\nCore:");
    println!("{}", core::Object::new().to_value().unwrap());
    println!("{}", core::Link::new().to_value().unwrap());
    println!("{}", core::Activity::new().to_value().unwrap());
    println!("{}", core::IntransitiveActivity::new().to_value().unwrap());
    println!("{}", core::Collection::new().to_value().unwrap());
    println!("{}", core::OrderedCollection::new().to_value().unwrap());
    println!("{}", core::CollectionPage::new().to_value().unwrap());
    println!("{}", core::OrderedCollectionPage::new().to_value().unwrap());

    println!("\nActivity:");
    println!("{}", activity::Accept::new().to_value().unwrap());
    println!("{}", activity::Add::new().to_value().unwrap());
    println!("{}", activity::Announce::new().to_value().unwrap());
    println!("{}", activity::Arrive::new().to_value().unwrap());
    println!("{}", activity::Block::new().to_value().unwrap());
    println!("{}", activity::Create::new().to_value().unwrap());
    println!("{}", activity::Delete::new().to_value().unwrap());
    println!("{}", activity::Dislike::new().to_value().unwrap());
    println!("{}", activity::Flag::new().to_value().unwrap());
    println!("{}", activity::Follow::new().to_value().unwrap());
    println!("{}", activity::Ignore::new().to_value().unwrap());
    println!("{}", activity::Invite::new().to_value().unwrap());
    println!("{}", activity::Join::new().to_value().unwrap());
    println!("{}", activity::Leave::new().to_value().unwrap());
    println!("{}", activity::Like::new().to_value().unwrap());
    println!("{}", activity::Listen::new().to_value().unwrap());
    println!("{}", activity::Move::new().to_value().unwrap());
    println!("{}", activity::Offer::new().to_value().unwrap());
    println!("{}", activity::Question::new().to_value().unwrap());
    println!("{}", activity::Read::new().to_value().unwrap());
    println!("{}", activity::Reject::new().to_value().unwrap());
    println!("{}", activity::Remove::new().to_value().unwrap());
    println!("{}", activity::TentativeAccept::new().to_value().unwrap());
    println!("{}", activity::TentativeReject::new().to_value().unwrap());
    println!("{}", activity::Travel::new().to_value().unwrap());
    println!("{}", activity::Undo::new().to_value().unwrap());
    println!("{}", activity::Update::new().to_value().unwrap());
    println!("{}", activity::View::new().to_value().unwrap());

    println!("\nActor:");
    println!("{}", actor::Actor::new().to_value().unwrap());
    println!("{}", actor::Application::new().to_value().unwrap());
    println!("{}", actor::Group::new().to_value().unwrap());
    println!("{}", actor::Organization::new().to_value().unwrap());
    println!("{}", actor::Person::new().to_value().unwrap());
    println!("{}", actor::Service::new().to_value().unwrap());

    println!("\nObject:");
    println!("{}", object::Article::new().to_value().unwrap());
    println!("{}", object::Audio::new().to_value().unwrap());
    println!("{}", object::Document::new().to_value().unwrap());
    println!("{}", object::Event::new().to_value().unwrap());
    println!("{}", object::Image::new().to_value().unwrap());
    println!("{}", object::Mention::new().to_value().unwrap());
    println!("{}", object::Note::new().to_value().unwrap());
    println!("{}", object::Page::new().to_value().unwrap());
    println!("{}", object::Place::new().to_value().unwrap());
    println!("{}", object::Profile::new().to_value().unwrap());
    println!("{}", object::Relationship::new().to_value().unwrap());
    println!("{}", object::Tombstone::new().to_value().unwrap());
    println!("{}", object::Video::new().to_value().unwrap());
}
