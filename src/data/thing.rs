use std::fmt::{Display, Formatter};
use uuid::Uuid;

/// Essentially the "core" type in Lever. Everything that the system cares about is a [`Thing`].
#[derive(Debug, async_graphql::SimpleObject, sqlx::FromRow)]
pub struct Thing {
    id: Uuid,
    kind: String,
    status: String,
}

impl Thing {
    /// Instantiate a new [`Thing`]. The [`Thing::id`] field is supplied automatically.
    pub fn new(kind: String, status: String) -> Self {
        let id = Uuid::new_v4();

        Self { id, kind, status }
    }
}

impl Display for Thing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
