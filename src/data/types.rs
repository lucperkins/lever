use async_graphql::scalar;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Data(Map<String, Value>);

impl Data {
    pub fn is_valid(&self) -> bool {
        !self.0.is_empty()
    }
}

scalar!(Data);
