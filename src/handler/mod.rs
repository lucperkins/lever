pub mod event {
    use crate::data::Event;
    use crate::error::Error;
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
                if let Err(errs) = self.handle(&event).await {
                    for err in errs {
                        println!("{:?}", err);
                    }
                }
            }
        }

        async fn handle(&self, event: &Event) -> Result<(), Vec<Error>> {
            let mut errors: Vec<Error> = vec![];

            for handler in &self.handlers {
                if let Err(err) = handler.handle(&event).await {
                    errors.push(err);
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
