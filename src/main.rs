#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod database;

use database::{
    products::{query_products,add_product_api, AddProduct,delete_product_api,DeleteProduct, ProductDTO},
    users::{check, login_user, register_user, UserData},
    MyError,
};
use dotenv::dotenv;

use rocket::serde::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[post("/register", format = "json", data = "<data>")]
async fn register(data: Json<UserData>) -> Result<String, MyError> {
    let mut result: String = "".to_string();
    if let Some (token) = &data.token {
        result = check(&token).await?;
    }

    if result != "administrator" {
        return Err(MyError::ForbiddenError("".to_string()));
    }

    register_user(data).await?;

    Ok("user registered".to_string())
}

#[post("/login", format = "json", data = "<data>")]
async fn login(data: Json<UserData>) -> Result<String, MyError> {
    let token = login_user(data).await?;
    Ok(token)
}

#[post("/get_products", data = "<token>")]
async fn get_products(token: String) -> Result<Json<Vec<ProductDTO>>, MyError> {
    check(&token).await?;
    let products = query_products().await?;
    Ok(products)
}

#[post("/add_product",format="json", data = "<product>")]
async fn add_product(product: Json<AddProduct>) -> Result<String, MyError> {
    let result = check(&product.token).await?;
    if result == "guest" {
        return Err(MyError::ForbiddenError("".to_string()));
    }

    add_product_api(product).await?;

    Ok("product added".to_string())
}

#[post("/delete_product",format="json", data = "<product>")]
async fn delete_product(product: Json<DeleteProduct>) -> Result<String, MyError> {
    let result = check(&product.token).await?;
    if result == "guest" {
        return Err(MyError::ForbiddenError("".to_string()));
    }
    delete_product_api(product.id).await?;
    Ok("product deleted".to_string())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allow_credentials(true);

    rocket::build()
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![login, register, get_products,add_product,delete_product])
        .launch()
        .await
        .unwrap();
}
