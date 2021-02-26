use bytes::Bytes;
use lever::data::{Event, Thing};
use std::collections::HashMap;
use uuid::Uuid;

fn main() {
    let thing_id = Uuid::new_v4();
    let mut metadata = HashMap::new();
    metadata.insert("item".into(), "tickle-me-elmo".into());
    let data = Bytes::from("something");
    let event = Event::new(thing_id, "sale".into(), None, Some(data));
    println!("{:?}", event);

    let thing = Thing::new(
        "item".into(),
        Some(metadata),
        Some(Bytes::from("here is some info")),
    );
    println!("{:?}", thing);
}
