use crate::data::Data;
use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

/// The core event type in Lever. An [`Event`] is anything meaningful thing that happens to a
/// [`thing::Thing`] in the system.
#[derive(Debug, async_graphql::SimpleObject, sqlx::FromRow)]
pub struct Event {
    pub id: Uuid,
    pub thing_id: Uuid,
    pub kind: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<Data>,
}

#[derive(async_graphql::InputObject)]
pub struct CreateEventInput {
    thing_id: Uuid,
    kind: String,
    metadata: Option<Data>,
}

impl CreateEventInput {
    pub fn new(thing_id: Uuid, kind: String, metadata: Option<Data>) -> Self {
        Self {
            thing_id,
            kind,
            metadata,
        }
    }
}

impl CreateEventInput {
    pub fn is_valid(&self) -> bool {
        !self.kind.is_empty()
    }
}

impl Event {
    /// Instantiate a new [`Event`]. The [`Event::id`] and [`Event::timestamp`] fields are supplied
    /// automatically.
    pub fn new(input: CreateEventInput) -> Self {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();

        Self {
            id,
            thing_id: input.thing_id,
            kind: input.kind,
            timestamp,
            metadata: input.metadata,
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
