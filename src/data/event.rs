use bytes::Bytes;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

/// The core event type in Lever. An [`Event`] is anything meaningful thing that happens to a
/// [`thing::Thing`] in the system.
#[derive(Debug, sqlx::FromRow)]
pub struct Event {
    id: Uuid,
    thing_id: Uuid,
    kind: String,
    timestamp: DateTime<Utc>,
    metadata: Option<HashMap<String, String>>,
    data: Option<Bytes>,
}

impl Event {
    /// Instantiate a new [`Event`]. The [`Event::id`] and [`Event::timestamp`] fields are supplied
    /// automatically.
    pub fn new(
        thing_id: Uuid,
        kind: String,
        metadata: Option<HashMap<String, String>>,
        data: Option<Bytes>,
    ) -> Self {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();

        Self {
            id,
            thing_id,
            kind,
            timestamp,
            metadata,
            data,
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
