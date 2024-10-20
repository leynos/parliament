use indradb::Identifier;
use indradb_proto::{Client, ClientError};
use uuid::Uuid;

pub trait Entity {
    const ENTITY_TYPE_NAME: &'static str;

    fn new(name: String) -> Self;
    async fn save(&self, client: Client) -> Result<bool, ClientError>;
    fn entity_type(&self) -> Identifier {
        return Identifier::new(Self::ENTITY_TYPE_NAME).unwrap();
    }
    fn get(client: Client, id: Uuid) -> Self;
}
