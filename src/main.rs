#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod database;

use database::{
    products::{add_product_api, add_property_api, delete_product_api, delete_property_api, modify_product_api, query_products, AddProduct, AddProperty, DeleteProduct, DeleteProperty, ModifyProduct, ProductDTO},
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
        return Err(MyError::ForbiddenError("you don't have permission".to_string()));
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
        return Err(MyError::ForbiddenError("you don't have permission".to_string()));
    }
    delete_product_api(product.id).await?;
    Ok("product deleted".to_string())
}

#[post("/modify_product",format="json", data = "<product>")]
async fn modify_product(product:Json<ModifyProduct>) -> Result<String,MyError> {
    let result = check(&product.token).await?;
    if result == "guest" {
        return Err(MyError::ForbiddenError("you don't have permission".to_string()));
    }
    modify_product_api(product).await?;
    Ok("product modified".to_string())
}

#[post("/add_property",format="json", data = "<property>")]
async fn add_property(property: Json<AddProperty>) -> Result<String, MyError> {
    let result = check(&property.token).await?;
    if result == "guest" {
        return Err(MyError::ForbiddenError("you don't have permission".to_string()));
    }
    add_property_api(property).await?;
    Ok("property added".to_string())
}

#[post("/delete_property",format="json", data = "<property>")]
async fn delete_property(property: Json<DeleteProperty>) -> Result<String, MyError> {
    let result = check(&property.token).await?;
    if result == "guest" {
        return Err(MyError::ForbiddenError("you don't have permission".to_string()));
    }
    delete_property_api(property).await?;
    Ok("property added".to_string())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allow_credentials(true);

    rocket::build()
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![login, register, get_products,add_product,delete_product, modify_product, delete_property, add_property])
        .launch()
        .await
        .unwrap();
}
