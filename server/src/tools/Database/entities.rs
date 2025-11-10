use rusqlite::Row;

pub trait FromRow {
    fn from_row(row: &Row) -> Self;
}


#[derive(Debug)]
pub struct UserEntity {
}