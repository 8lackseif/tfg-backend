use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use thiserror::Error;

pub mod users;
pub mod products;
pub mod tags;
pub mod stock;
pub mod migration;

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
    #[error("You don't have permission to access.")]
    #[response(status=403)]
    ForbiddenError(String)
    
}

impl From<sqlx::Error> for MyError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(_) => MyError::ConstraintError(value.to_string()),
            _ => MyError::InternalError("Internal error".to_string()),
        }
    }
}

impl From<jwt::Error> for MyError {
    fn from(value: jwt::Error) -> Self {
        match value {
            jwt::Error::InvalidSignature => MyError::ForbiddenError(value.to_string()),
            _ => MyError::InternalError("Internal error".to_string()),
        }
    }
}

