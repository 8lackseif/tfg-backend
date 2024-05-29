#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod database;

use database::{
    products::{add_product_api, add_property_api, delete_product_api, delete_property_api, modify_product_api, query_products,
                AddProduct, AddProperty, DeleteProduct, DeleteProperty, ModifyProduct, ProductDTO}, 
                stock::{add_stocks, change_stocks, get_stock_history_api, get_stocks_api, add_stock_from_code,AddStocks, StockDto, StockHistory, ProductStockHistory}, 
                tags::{add_tag_api, bind_tag_api, delete_tag_api, query_tags, unbind_tag_api, get_popular_tags_api, ModifyProductToTag, ModifyTags, TagsDTO, PopularTag}, 
                users::{check, login_user, register_user, UserData},
                migration::{ExportingDTO, Importing, export_api, import_api},
                MyError
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

    add_product_api(&product).await?;
    add_stock_from_code(&product.code, product.stock).await?;


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

#[post("/get_tags", data = "<token>")]
async fn get_tags(token:String) -> Result<Json<Vec<TagsDTO>>, MyError> {
    check(&token).await?;
    let tags = query_tags().await?;
    Ok(tags)
}

#[post("/delete_tag", format = "json", data = "<tag>")]
async fn delete_tag(tag: Json<ModifyTags>) -> Result<String, MyError> {
    let result = check(&tag.token).await?;
    if result == "guest" {
        return Err(MyError::ForbiddenError("you don't have permission".to_string()));
    }
    delete_tag_api(tag).await?;
    Ok("tag deleted".to_string())
}

#[post("/add_tag", format = "json", data = "<tag>")]
async fn add_tag(tag: Json<ModifyTags>) -> Result<String, MyError> {
    let result = check(&tag.token).await?;
    if result == "guest" {
        return Err(MyError::ForbiddenError("you don't have permission".to_string()));
    }
    add_tag_api(tag).await?;
    Ok("tag added".to_string())
}

#[post("/bind_tag", format = "json", data = "<tag>")]
async fn bind_tag(tag: Json<ModifyProductToTag>) -> Result<String, MyError> {
    let result = check(&tag.token).await?;
    if result == "guest" {
        return Err(MyError::ForbiddenError("you don't have permission".to_string()));
    }
    bind_tag_api(tag).await?;
    Ok("tag binded".to_string())
}

#[post("/unbind_tag", format = "json", data = "<tag>")]
async fn unbind_tag(tag: Json<ModifyProductToTag>) -> Result<String, MyError> {
    let result = check(&tag.token).await?;
    if result == "guest" {
        return Err(MyError::ForbiddenError("you don't have permission".to_string()));
    }
    unbind_tag_api(tag).await?;
    Ok("tag unbinded".to_string())
}

#[post("/var_stock", format = "json", data = "<stocks_var>")]
async fn var_stock(stocks_var: Json<AddStocks>) -> Result<String, MyError> {
    let result = check(&stocks_var.token).await?;
    if result == "guest" {
        return Err(MyError::ForbiddenError("you don't have permission".to_string()));
    }
    change_stocks(&stocks_var.stocks).await?;
    add_stocks(&stocks_var.stocks).await?;
    Ok("stock changed".to_string())
}

#[post("/get_stocks", data = "<token>")]
async fn get_stocks(token:String) -> Result<Json<Vec<StockDto>>, MyError> {
    check(&token).await?;
    let stocks = get_stocks_api().await?;
    Ok(stocks)
}

#[post("/get_stock_history", format = "json", data = "<query>")]
async fn get_stock_history(query:Json<ProductStockHistory>) -> Result <Json<Vec<StockHistory>>,MyError> {
    check(&query.token).await?;
    let data = get_stock_history_api(query.id).await?;
    Ok(data)
}

#[post("/get_popular_tags", data = "<token>")]
async fn get_popular_tags(token:String) -> Result<Json<Vec<PopularTag>>, MyError> {
    check(&token).await?;
    let popular_tags = get_popular_tags_api().await?;
    Ok(popular_tags)
}

#[post("/export_data", data = "<token>")]
async fn export_data(token: String) -> Result<Json<ExportingDTO>, MyError> {
    check(&token).await?;
    let export = export_api().await?;
    Ok(export)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allow_credentials(true);

    rocket::build()
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![login, register,
                            get_products,add_product,delete_product, modify_product,
                            delete_property, add_property,
                            get_tags, delete_tag, add_tag, bind_tag, unbind_tag, get_popular_tags,
                            var_stock, get_stocks, get_stock_history,
                            export_data])
        .launch()
        .await
        .unwrap();
}
