use std::collections::HashMap;
use std::sync::Arc;

use crate::decode::Decode;
use crate::postgres::protocol::DataRow;
use crate::postgres::Postgres;
use crate::row::Row;
use crate::types::Type;

// 's: the lifetime of the database server connection or socket
pub struct PgRow<'s> {
    pub(super) data: DataRow<'s>,
    pub(super) columns: Arc<HashMap<Box<str>, usize>>,
}

impl<'s> Row<'s> for PgRow<'s> {
    type Database = Postgres;

    fn get<T>(&self, index: usize) -> T
    where
        T: Type<Self::Database>,
        T: Decode<'s, Self::Database>,
    {
        todo!()
    }
}
