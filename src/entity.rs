use indradb::{Identifier, VertexProperties};
use indradb_proto::{Client, ClientError};
use uuid::Uuid;

pub type EntityResult<T> = Result<Box<T>, Box<dyn std::error::Error>>;

pub trait Entity {
    const ENTITY_TYPE_NAME: &'static str;

    fn new(name: String) -> Self;
    async fn save(&self, client: &mut Client) -> Result<(), ClientError>;
    fn entity_type(&self) -> Identifier {
        return Identifier::new(Self::ENTITY_TYPE_NAME).unwrap();
    }
    async fn get(client: &mut Client, id: Uuid) -> EntityResult<Self>;
    fn from_vertex(vert: &VertexProperties) -> EntityResult<Self>;
}
