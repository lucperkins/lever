use async_graphql::scalar;
use async_graphql::validators::InputValueValidator;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

/// Essentially the "core" type in Lever. Everything that the system cares about is a [`Thing`].
#[derive(Debug, async_graphql::SimpleObject, sqlx::FromRow)]
pub struct Thing {
    pub id: Uuid,
    pub kind: String,
    pub status: String,
    pub metadata: Option<Metadata>,
}

#[derive(async_graphql::InputObject)]
pub struct ThingInput {
    pub kind: String,
    pub status: String,
    pub metadata: Option<Metadata>,
}

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Metadata(Map<String, Value>);

scalar!(Metadata);

impl InputValueValidator for ThingInput {
    fn is_valid(&self, _: &async_graphql::Value) -> Result<(), String> {
        if let Some(meta) = &self.metadata {
            if !meta.0.is_empty() {
                return Err("metadata can't be empty".to_owned());
            }
        }
        Ok(())
    }
}

impl Display for Thing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
