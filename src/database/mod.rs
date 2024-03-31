use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use thiserror::Error;

pub mod users;
pub mod products;

lazy_static!{
    static ref POOL: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&dotenv::var("DATABASE_URL").expect("failed to find DATABASE_URL on env")).unwrap();
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
    #[error("login failed")]
    #[response(status=401)]
    LoginError(String),
    #[error("user not found")]
    #[response(status=404)]
    UserNotFoundError(String),


}

impl From<sqlx::Error> for MyError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(_) => MyError::ConstraintError(value.to_string()),
            _ => MyError::InternalError("Internal error".to_string()),
        }
    }
}

