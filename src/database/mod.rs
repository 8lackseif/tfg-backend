use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub mod users;

lazy_static!{
    static ref POOL: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect_lazy("mysql://root:@localhost/my_database").unwrap();
}

pub enum MyError{
}
