use async_trait::async_trait;
use lever::data::Event;
use lever::handler::event::{Executor, Handler};
use lever::handler::Error;
use std::collections::HashMap;
use uuid::Uuid;

struct PrintHandler;

#[async_trait]
impl Handler for PrintHandler {
    async fn handle(&self, event: &Event) -> Result<(), Error> {
        println!("{}", event);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let handlers: Vec<Box<dyn Handler>> = vec![Box::new(PrintHandler)];
    let executor = Executor::new(handlers);

    let mut events: Vec<Event> = vec![];

    for n in 0..100000 {
        let mut metadata: HashMap<String, String> = HashMap::new();
        metadata.insert("item_id".into(), format!("{}", n));
        let event = Event::new(Uuid::new_v4(), "item".into(), Some(metadata), None);
        events.push(event);
    }

    let mut stream = tokio_stream::iter(events);

    Ok(executor.run(&mut stream).await)
}
