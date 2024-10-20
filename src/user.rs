// Serializes and deserializes users to IndraDB

use crate::entity::Entity;
use indradb::{Identifier, Json};
use indradb::{Vertex, VertexProperty};
use indradb_proto::{Client, ClientError};
use serde::{Deserialize, Serialize}; // Import serde traits
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl Entity for User {
    const ENTITY_TYPE_NAME: &'static str = "user";

    fn new(name: String) -> Self {
        User {
            id: uuid7::new_v7(),
            name: name,
        }
    }

    async fn save(&self, mut client: Client) -> Result<bool, ClientError> {
        let user_v = Vertex::with_id(self.id, self.entity_type());
        let name_json = Json::from(serde_json::to_value(&self.name).unwrap());
        // let name_prop = VertexProperty::new(self.id, name_json);
        let name_prop_id = Identifier::new("name").unwrap();
        client.create_vertex(&user_v).await?;
        let user_query = indradb::SpecificVertexQuery::single(self.id);
        client
            .set_properties(user_query, name_prop_id, &name_json)
            .await?;
        Ok(true)
    }

    async fn get(client: Client) -> Result<Self, ClientError> {
        
    }
}
