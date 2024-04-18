use rocket::serde::{json::Json, Deserialize, Serialize};

use super::{MyError, POOL};

#[derive(Debug,FromForm,Deserialize)]
pub struct AddProduct {
    pub code: String,
    pub name: String,
    pub description: String,
    pub stock: i32,
    pub token: String
}

#[derive(Debug,FromForm,Deserialize)]
pub struct DeleteProduct {
    pub token: String,
    pub id: i32
}

#[derive(Debug, Serialize)]
pub struct ProductDTO {
    id: i32,
    code: String,
    name: String,
    description: Option<String>,
    stock: Option<i32>,
    tags: Option<String>,
    property_names: Option<String>,
    property_values: Option<String>
}

pub async fn query_products() -> Result<Json<Vec<ProductDTO>>, MyError> {
    let pool = POOL.clone();
    let products = sqlx::query_as!(
        ProductDTO,
        "
        SELECT p.id, 
            p.code,
            p.name,
            p.description,
            p.stock,
            JSON_ARRAYAGG(IFNULL(t.name,'')) AS tags,
            pp.property_names,
            pp.property_values
            FROM 
                products p
            LEFT JOIN 
                productsTotags pt ON p.id = pt.productID
            LEFT JOIN 
                tags t ON pt.tagID = t.id
            LEFT JOIN (
                SELECT ProductID, JSON_ARRAYAGG(IFNULL(property,'')) AS property_names, JSON_ARRAYAGG(IFNULL(value,'')) AS property_values
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

pub async fn add_product_api(product: Json<AddProduct>) -> Result<(),MyError> {
    let pool = POOL.clone();
    sqlx::query!(
        "INSERT INTO products VALUES(0,?,?,?,?)",product.code, product.name, product.description, product.stock)
        .execute(&pool).await?;
    Ok(())
}

pub async fn delete_product_api(id: i32) -> Result<(),MyError> {
    let pool = POOL.clone();
    sqlx::query!(
        "DELETE FROM products where id = ?",id)
        .execute(&pool).await?;
    Ok(())
}