#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

mod database;

use database::{products::{query_products, Product}, users::{login_user, query_users, register_user, UserData}, MyError};
use dotenv::dotenv;

use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::{futures::task::ArcWake, serde::json::Json};
use sqlx::query_builder;

#[post("/register" ,format="json" ,data ="<data>")]
async fn register(data: Json<UserData>) -> Result<String,MyError> {
    register_user(&data.username,&data.pwd).await?;
    Ok("user registered".to_string())
} 



#[post("/login", format="json", data="<data>")]
async fn login(data: Json<UserData>) -> Result<String,MyError> {
    let token = login_user(&data.username, &data.pwd).await?;
    Ok(token)
}

#[get("/get_products")]
async fn get_products() -> Result<Json<Vec<Product>>,MyError> {
    let products = query_products().await?;
    Ok(products)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allow_credentials(true);

    rocket::build().attach(cors.to_cors().unwrap()).mount("/", routes![login,register,get_products]).launch().await.unwrap();
}

