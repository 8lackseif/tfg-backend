use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use thiserror::Error;

pub mod users;

lazy_static!{
    static ref POOL: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect_lazy("mysql://root:@localhost/my_database").unwrap();
}


#[derive(Error,Debug,Responder)]
#[response(content_type = "json")]
pub enum MyError{
    #[error("'[0]'")]
    #[response(status=400)]
    ConstraintError(String),
    #[error("Internal error")]
    #[response(status=500)]
    InternalError(String),

}

impl From<sqlx::Error> for MyError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(_) => MyError::ConstraintError(value.to_string()),
            _ => MyError::InternalError("Internal error".to_string()),
        }
    }
}

