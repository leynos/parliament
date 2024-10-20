// Serializes and deserializes users to IndraDB

use crate::entity::{Entity, EntityResult};
use indradb::{Identifier, Json, QueryExt, QueryOutputValue, VertexProperties};
use indradb::{Vertex, VertexProperty};
use indradb_proto::{Client, ClientError};
use serde::{Deserialize, Serialize}; // Import serde traits
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UserBody {
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

    async fn save(&self, client: &mut Client) -> Result<(), ClientError> {
        let user_v = Vertex::with_id(self.id, self.entity_type());
        let name_json = Json::from(serde_json::to_value(&self.name).unwrap());
        // let name_prop = VertexProperty::new(self.id, name_json);
        let name_prop_id = Identifier::new("name").unwrap();
        client.create_vertex(&user_v).await?;
        let user_query = indradb::SpecificVertexQuery::single(self.id);
        client
            .set_properties(user_query, name_prop_id, &name_json)
            .await?;
        Ok(())
    }

    fn from_vertex(vert: &VertexProperties) -> EntityResult<Self> {
        let name_prop_id = Identifier::new("name").unwrap();
        let name_prop = vert.props.iter().find(|&prop| prop.name == name_prop_id);
        if name_prop.is_none() {
            return Err("No name :(".into());
        }
        let name_option = name_prop.unwrap().value.as_str();
        if name_option.is_none() {
            return Err("Noooo! :(".into());
        }
        Ok(Box::new(User {
            id: vert.vertex.id,
            name: name_option.unwrap().to_string(),
        }))
    }

    async fn get( client: &mut Client, id: Uuid) -> EntityResult<Self> {
        let user_query = indradb::SpecificVertexQuery::single(id);
        let user_properties_query = user_query.properties()?;
        let res = client.get(user_properties_query).await?;
        match res.first() {
            Some(QueryOutputValue::VertexProperties(vertices)) => match vertices.first() {
                Some(vert) => Self::from_vertex(vert),
                None => Err("Int borked".into()),
            },
            Some(_) => Err("It borked".into()),
            None => Err("It borked some more".into()),
        }
    }
}
