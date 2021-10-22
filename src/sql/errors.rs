use derive_more::{Display, Error};

pub struct MyError {
    name: &'static str,
}


#[derive(Debug, Display, Error)]
pub enum DbError {
    #[display(fmt = "{} record not found", record)]
    RecordNotFound {record: String},
}

// impl error::ResponseError for DbError {}