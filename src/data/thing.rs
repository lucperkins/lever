use bytes::Bytes;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

/// Essentially the "core" type in Lever. Everything that the system cares about is a [`Thing`].
#[derive(Debug)]
pub struct Thing {
    id: Uuid,
    kind: String,
    metadata: Option<HashMap<String, String>>,
    data: Option<Bytes>,
}

impl Thing {
    /// Instantiate a new [`Thing`]. The [`Thing::id`] field is supplied automatically.
    pub fn new(
        kind: String,
        metadata: Option<HashMap<String, String>>,
        data: Option<Bytes>,
    ) -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            kind,
            metadata,
            data,
        }
    }
}

impl Display for Thing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
