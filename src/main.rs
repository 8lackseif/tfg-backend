#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

mod database;

use database::{users::{query_users, register_user, UserData}, MyError};
use dotenv::dotenv;

use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::serde::json::Json;

#[post("/register" ,format="json" ,data ="<data>")]
async fn register(data: Json<UserData>) -> Result<String,MyError> {
    register_user(&data.username,&data.pwd).await?;
    Ok("user registered".to_string())
} 



#[post("/login")]
async fn login() -> String {
    query_users().await;
    "hola".to_string()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allow_credentials(true);

    rocket::build().attach(cors.to_cors().unwrap()).mount("/", routes![login,register]).launch().await.unwrap();
}

