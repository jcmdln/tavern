use tavern::link::{Link, Mention};
use tavern::object::{
    Article, Audio, Document, Event, Image, Note, Object, Page, Place, Profile, Relationship,
    Tombstone, Video,
};

fn main() {
    println!("{}", serde_json::to_value(Link::builder("https://domain.tld")).unwrap());
    println!("{}", serde_json::to_value(Mention::builder("https://domain.tld")).unwrap());

    println!("{}", serde_json::to_value(Object::builder()).unwrap());
    println!("{}", serde_json::to_value(Article::builder()).unwrap());
    println!("{}", serde_json::to_value(Audio::builder()).unwrap());
    println!("{}", serde_json::to_value(Document::builder()).unwrap());
    println!("{}", serde_json::to_value(Event::builder()).unwrap());
    println!("{}", serde_json::to_value(Image::builder()).unwrap());
    println!("{}", serde_json::to_value(Note::builder()).unwrap());
    println!("{}", serde_json::to_value(Page::builder()).unwrap());
    println!("{}", serde_json::to_value(Place::builder()).unwrap());
    println!("{}", serde_json::to_value(Profile::builder()).unwrap());
    println!("{}", serde_json::to_value(Relationship::builder()).unwrap());
    println!("{}", serde_json::to_value(Tombstone::builder()).unwrap());
    println!("{}", serde_json::to_value(Video::builder()).unwrap());
}
