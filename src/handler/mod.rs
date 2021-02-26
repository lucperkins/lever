#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub mod event {
    use super::Error;
    use crate::data::Event;
    use async_trait::async_trait;
    use tokio_stream::StreamExt;

    #[async_trait]
    pub trait Handler {
        async fn handle(&self, event: &Event) -> Result<(), Error>;
    }

    pub struct Executor {
        handlers: Vec<Box<dyn Handler>>,
    }

    impl Executor {
        pub fn new(handlers: Vec<Box<dyn Handler>>) -> Self {
            Self { handlers }
        }

        pub async fn run<S: StreamExt<Item = Event> + Unpin>(&self, event_stream: &mut S) {
            println!("Starting up the executor");

            while let Some(event) = event_stream.next().await {
                match self.handle(&event).await {
                    Ok(_) => println!("success"),
                    Err(errs) => {
                        println!("failure");
                        for err in errs {
                            println!("  {:?}", err);
                        }
                    }
                }
            }
        }

        async fn handle(&self, event: &Event) -> Result<(), Vec<Error>> {
            let mut errors: Vec<Error> = vec![];

            for handler in &self.handlers {
                match handler.handle(&event).await {
                    Err(e) => errors.push(e),
                    _ => (),
                }
            }

            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors)
            }
        }
    }
}
