use rocket::serde::{self, Deserialize, Serialize,json::Json};

use super::{MyError, POOL};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::{collections::BTreeMap, fmt::format};


#[derive(Debug,Serialize)]
pub struct Product {
    id : i32,
    code: String,
    name : String,
    description : Option<String>,
    stock : Option<i32>
}

#[derive(Debug,Serialize)]
pub struct Products {
    products: Vec<Product>,
}

pub async fn query_products()-> Result<Json<Products>,MyError> { 
    let pool = POOL.clone();
    let products = sqlx::query_as!(Product,
        "SELECT * FROM products").fetch_all(&pool).await?;
    Ok(Json(Products{products: products}))
}

