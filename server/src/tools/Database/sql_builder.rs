use crate::common_definitions::senka_error::SenkaError;

pub trait SqlExecutable {
    fn build(&self) -> Result<String, SenkaError>;
}

pub struct Queryable;

pub struct Insertable;

pub struct Updateable;

pub struct Deleteable;

pub struct Storageable;