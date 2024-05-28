use super::{MyError, POOL};
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ExportingDTO {
    users: Vec<ExportingUserDTO>,
    products: Vec<ExportingProductDTO>
}

#[derive(Debug, Serialize)]
pub struct ExportingProductDTO {
    code: String,
    name: String,
    description: Option<String>,
    stock: Option<i32>,
    image_url: Option<String>,
    tags: Option<String>,
    properties: Option<String>,
    stock_var: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExportingUserDTO {
    username: String,
    pwd: String,
    rol: String
}

#[derive(Debug, FromForm, Deserialize)]
pub struct Importing {
    pub products: Vec<ImportingProduct>,
    pub users: Vec<ImportingUser>
}

#[derive(Debug, FromForm, Deserialize)]
pub struct ImportingUser {
    pub username: String,
    pub pwd: String,
    pub rol: String
}

#[derive(Debug, FromForm, Deserialize)]
pub struct ImportingProduct {
    pub code: String,
    pub name: String,
    pub description: String,
    pub stock: i32,
    pub image_url: String,
    pub tags: Vec<String>,
    pub properties: Vec<ImportingProperty>,
    pub stock_var: Vec<ImportingStockVar>,
}

#[derive(Debug, FromForm, Deserialize)]
pub struct ImportingProperty {
    pub property: String,
    pub value: String,
}

#[derive(Debug, FromForm, Deserialize)]
pub struct ImportingStockVar {
    pub var_date: String,
    pub quantity: i32,
}

/* 
pub async fn import_api() -> Result<Json<Vec<ExportingDTO>>, MyError> {
    let query = "
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
                p.id;
    ";

    let pool = POOL.clone();
    let products = sqlx::query_as!(ExportingProductDTO, query).fetch_all(&pool).await?;
    Ok(Json())
}
*/
