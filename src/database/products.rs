use rocket::serde::{self, json::Json, Deserialize, Serialize};

use super::{MyError, POOL};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::format,
};

#[derive(Debug, Serialize)]
pub struct Product {
    id: i32,
    code: String,
    name: String,
    description: Option<String>,
    stock: Option<i32>,
    tags: Option<String>,
    propertyNames: Option<String>,
    propertyValues: Option<String>
}

pub async fn query_products() -> Result<Json<Vec<Product>>, MyError> {
    let pool = POOL.clone();
    let products = sqlx::query_as!(
        Product,
        "
        SELECT p.id, 
            p.code,
            p.name,
            p.description,
            p.stock,
            JSON_ARRAYAGG(IFNULL(t.name,'')) AS tags,
            pp.propertyNames,
            pp.propertyValues
            FROM 
                products p
            LEFT JOIN 
                productsTotags pt ON p.id = pt.productID
            LEFT JOIN 
                tags t ON pt.tagID = t.id
            LEFT JOIN (
                SELECT ProductID, JSON_ARRAYAGG(IFNULL(property,'')) AS propertynames, JSON_ARRAYAGG(IFNULL(value,'')) AS propertyvalues
                FROM properties
                GROUP BY ProductID
                ) pp ON pp.ProductID = p.id 
            GROUP BY
                p.id;"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(products))
}
