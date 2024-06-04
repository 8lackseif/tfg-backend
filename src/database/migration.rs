use super::{MyError, POOL};
use rocket::serde::{json::Json, Deserialize, Serialize};
use sqlx::{MySql, Pool};

#[derive(Debug, Serialize)]
pub struct ExportingDTO {
    users: Vec<ExportingUserDTO>,
    products: Vec<ExportingProductDTO>,
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
    rol: String,
}

struct IdStruct {
    id: i32,
}

#[derive(Debug, FromForm, Deserialize)]
pub struct Importing {
    pub token: String,
    pub products: Vec<ImportingProduct>,
    pub users: Vec<ImportingUser>,
}

#[derive(Debug, FromForm, Deserialize)]
pub struct ImportingUser {
    pub username: String,
    pub pwd: String,
    pub rol: String,
}

#[derive(Debug, FromForm, Deserialize)]
pub struct ImportingProduct {
    pub code: String,
    pub name: String,
    pub description: String,
    pub stock: i32,
    pub image_url: String,
    pub tags: Vec<Option<String>>,
    pub properties: Option<Vec<ImportingProperty>>,
    pub stock_var: Option<Vec<ImportingStockVar>>,
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

pub async fn export_api() -> Result<Json<ExportingDTO>, MyError> {
    let pool = POOL.clone();
    let products = sqlx::query_as!(ExportingProductDTO,
        "SELECT  p.code,
            p.name,
            p.description,
            p.stock,
            p.image_url,
            JSON_ARRAYAGG(t.name) AS tags,
            pp.properties,
            sv.stock_var
        FROM    products p
        LEFT JOIN 
            productsTotags pt ON p.id = pt.productID
        LEFT JOIN 
            tags t ON pt.tagID = t.id
        LEFT JOIN (
            SELECT ProductID, JSON_ARRAYAGG(JSON_OBJECT('property', property, 'value', value)) as properties
            FROM properties
            GROUP BY ProductID
        ) pp ON pp.ProductID = p.id
        LEFT JOIN (
            SELECT productID, JSON_ARRAYAGG(JSON_OBJECT('var_date', varDate, 'quantity', quantity)) as stock_var
            FROM stockVar
            GROUP BY ProductID
        ) sv ON sv.ProductID = p.id
        GROUP BY p.id")
        .fetch_all(&pool).await?;

    let users = sqlx::query_as!(
        ExportingUserDTO,
        "SELECT username, pwd, rol FROM users WHERE username != 'admin'"
    )
    .fetch_all(&pool)
    .await?;

    let export = ExportingDTO {
        products: products,
        users: users,
    };

    Ok(Json(export))
}

pub async fn import_api(import: Json<Importing>) -> Result<(), MyError> {
    let pool = POOL.clone();
    let transaction = pool.begin().await?;

    match importing(import, pool).await {
        Err(e) => {
            transaction.rollback().await?;
            return Err(e);
        }
        Ok(_) => {}
    };

    transaction.commit().await?;

    Ok(())
}

async fn importing(import: Json<Importing>, pool: Pool<MySql>) -> Result<(), MyError> {
    import_users(&import.users, &pool).await?;
    import_products(&import.products, &pool).await?;
    Ok(())
}

async fn import_users(users: &Vec<ImportingUser>, pool: &Pool<MySql>) -> Result<(), MyError> {
    let mut query = "INSERT INTO users VALUES ".to_string();
    let query2 = users
        .iter()
        .map(|u| format!("(0,'{}','{}','{}',0),", u.username, u.pwd, u.rol))
        .reduce(|acc, e| acc + &e);

    if let Some(mut q) = query2 {
        q.pop();
        query += &q;
    }
    sqlx::query(&query).execute(pool).await?;

    Ok(())
}

async fn import_products(
    products: &Vec<ImportingProduct>,
    pool: &Pool<MySql>,
) -> Result<(), MyError> {
    for p in products {
        sqlx::query!(
            "INSERT INTO products VALUES(0,?,?,?,?,?)",
            p.code,
            p.name,
            p.description,
            p.stock,
            p.image_url
        )
        .execute(pool)
        .await?;
        let pid = sqlx::query_as!(IdStruct, "SELECT id FROM products WHERE code = ?", p.code)
            .fetch_one(pool)
            .await?;

        for t in &p.tags {
            if let Some(tag) = t {
                import_tag(pid.id, tag.to_string(), pool).await?;
            }
        }

        if let Some(pp) = &p.properties {
            import_properties(pid.id, &pp, pool).await?;
        }

        if let Some(sv) = &p.stock_var {
            import_stock_var(pid.id, &sv, pool).await?;
        }
    }

    Ok(())
}

async fn import_tag(product_id: i32, tag_name: String, pool: &Pool<MySql>) -> Result<(), MyError> {
    let tag = sqlx::query_as!(IdStruct, "SELECT id FROM tags WHERE name = ?", tag_name)
        .fetch_optional(pool)
        .await?;
    let tag_id: i32;
    if let Some(id_struct) = tag {
        tag_id = id_struct.id;
    } else {
        sqlx::query!("INSERT INTO tags VALUES(0,?)", tag_name)
            .execute(pool)
            .await?;
        let generated_tag =
            sqlx::query_as!(IdStruct, "SELECT id FROM tags WHERE name = ?", tag_name)
                .fetch_one(pool)
                .await?;
        tag_id = generated_tag.id;
    }
    sqlx::query!("INSERT INTO productsTotags VALUES(?,?)", product_id, tag_id)
        .execute(pool)
        .await?;

    Ok(())
}

async fn import_properties(
    product_id: i32,
    properties: &Vec<ImportingProperty>,
    pool: &Pool<MySql>,
) -> Result<(), MyError> {
    let mut query = "INSERT INTO properties VALUES ".to_string();
    let query2 = properties
        .iter()
        .map(|p| format!("({},'{}','{}'),", product_id, p.property, p.value))
        .reduce(|acc, e| acc + &e);

    if let Some(mut q) = query2 {
        q.pop();
        query += &q;
    }
    sqlx::query(&query).execute(pool).await?;

    Ok(())
}

async fn import_stock_var(
    product_id: i32,
    stock_var: &Vec<ImportingStockVar>,
    pool: &Pool<MySql>,
) -> Result<(), MyError> {
    let mut query = "INSERT INTO stockVar VALUES ".to_string();
    let query2 = stock_var
        .iter()
        .map(|sv| format!("(0,{},'{}',{}),", product_id, sv.var_date, sv.quantity))
        .reduce(|acc, e| acc + &e);

    if let Some(mut q) = query2 {
        q.pop();
        query += &q;
    }
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}
