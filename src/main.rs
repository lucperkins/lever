use bytes::Bytes;
use lever::data::Event;
use std::collections::HashMap;
use uuid::Uuid;

fn main() {
    let thing_id = Uuid::new_v4();
    let mut metadata = HashMap::new();
    metadata.insert("item".into(), "tickle-me-elmo".into());
    let data = Bytes::from("something");
    let event = Event::new(thing_id, "sale".into(), Some(metadata), Some(data));
    println!("{:?}", event);
}
