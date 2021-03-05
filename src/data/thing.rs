use super::types::Data;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

/// Essentially the "core" type in Lever. Everything that the system cares about is a [`Thing`].
#[derive(Debug, async_graphql::SimpleObject, sqlx::FromRow)]
pub struct Thing {
    pub id: Uuid,
    pub kind: String,
    pub status: String,
    pub metadata: Option<Data>,
    pub data: Data,
}

#[derive(async_graphql::InputObject)]
pub struct ThingInput {
    pub kind: String,
    pub status: String,
    pub metadata: Option<Data>,
    pub data: Data,
}

impl Display for Thing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
