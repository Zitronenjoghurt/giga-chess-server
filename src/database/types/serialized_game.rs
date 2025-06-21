use diesel::deserialize::FromSql;
use diesel::pg::sql_types::Bytea;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use giga_chess::prelude::Game;
use std::error::Error;
use std::fmt::Debug;
use std::io::Write;

#[derive(Debug, Clone, Eq, PartialEq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Bytea)]
pub struct SerializedGame(pub Game);

impl SerializedGame {
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        bincode::encode_to_vec(&self.0, bincode::config::standard())
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let (game, _) = bincode::decode_from_slice(bytes, bincode::config::standard())
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        Ok(SerializedGame(game))
    }
}

impl ToSql<Bytea, Pg> for SerializedGame {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let bytes = self.serialize()?;
        out.write_all(&bytes)?;
        Ok(IsNull::No)
    }
}

impl FromSql<Bytea, Pg> for SerializedGame {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        let bytes = bytes.as_bytes();
        SerializedGame::deserialize(bytes)
    }
}
