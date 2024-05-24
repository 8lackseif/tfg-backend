use rocket::serde::{json::Json, Deserialize, Serialize};

use super::{MyError, POOL};

#[derive(Debug,FromForm,Deserialize)]
pub struct AddProduct {
    pub code: String,
    pub name: String,
    pub description: String,
    pub stock: i32,
    pub token: String,
    pub image_url: String
}

#[derive(Debug,FromForm,Deserialize)]
pub struct DeleteProduct {
    pub token: String,
    pub id: i32
}

#[derive(Debug,FromForm,Deserialize)]
pub struct ModifyProduct {
    pub token: String,
    pub id: i32,
    pub code: String,
    pub name: String,
    pub description: String,
    pub image_url: String
}

#[derive(Debug,FromForm,Deserialize)]
pub struct AddProperty {
    pub token: String,
    pub id: i32,
    pub property_name: String,
    pub property_value: String
}

#[derive(Debug,FromForm,Deserialize)]
pub struct DeleteProperty {
    pub token: String,
    pub id: i32,
    pub property_name: String,
}

#[derive(Debug, Serialize)]
pub struct ProductDTO {
    id: i32,
    code: String,
    name: String,
    description: Option<String>,
    stock: Option<i32>,
    tags: Option<String>,
    image_url: Option<String>,
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
            p.image_url,
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

pub async fn add_product_api(product: &Json<AddProduct>) -> Result<(),MyError> {
    let pool = POOL.clone();
    sqlx::query!(
        "INSERT INTO products VALUES(0,?,?,?,?,?)",product.code, product.name, product.description, product.stock, product.image_url)
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

pub async fn modify_product_api(product: Json<ModifyProduct>) -> Result<(),MyError> {
    let pool = POOL.clone();
    sqlx::query!(
        "UPDATE products SET code = ?, name = ?, description = ?, image_url = ? where id = ?",product.code, product.name, product.description, product.image_url, product.id)
        .execute(&pool).await?;
    Ok(())
}

pub async fn add_property_api(property: Json<AddProperty>) -> Result<(),MyError> {
    let pool = POOL.clone();
    sqlx::query!(
        "INSERT INTO properties VALUES(?,?,?)", property.id, property.property_name, property.property_value)
        .execute(&pool).await?;
    Ok(())
}

pub async fn delete_property_api(property: Json<DeleteProperty>) -> Result<(), MyError> {
    let pool = POOL.clone();
    sqlx::query!(
        "DELETE FROM properties WHERE productid = ? and property = ?", property.id, property.property_name)
        .execute(&pool).await?;
    Ok(())
}