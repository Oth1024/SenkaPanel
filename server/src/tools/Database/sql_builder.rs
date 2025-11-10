use crate::common_definitions::senka_error::SenkaError;

pub trait SqlBuilder {
    fn build(&self) -> Result<String, SenkaError>;
}

pub struct SelectBuilder;

pub struct InsertBuilder;

pub struct UpdateBuilder;

pub struct DeleteBuilder;