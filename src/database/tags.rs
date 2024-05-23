use rocket::serde::{json::Json, Deserialize, Serialize};

use super::{MyError, POOL};


#[derive(Debug,Serialize)]
pub struct TagsDTO {
    name: String
}

#[derive(Debug,Serialize)]
pub struct PopularTag {
    tag_name: Option<String>,
    count: i64
}

#[derive(Debug,FromForm,Deserialize)]
pub struct ModifyTags {
    pub token: String,
    pub name: String
}

#[derive(Debug,FromForm,Deserialize)]
pub struct ModifyProductToTag {
    pub token: String,
    pub product_id: i32,
    pub name: String
}

pub async fn query_tags() -> Result<Json<Vec<TagsDTO>>,MyError> {
    let pool = POOL.clone();
    let tags = sqlx::query_as!(TagsDTO,
        "SELECT name FROM tags")
        .fetch_all(&pool).await?;
    Ok(Json(tags))
}

pub async fn delete_tag_api(tag: Json<ModifyTags>) -> Result<(),MyError> {
    let pool = POOL.clone();
    sqlx::query!(
        "DELETE FROM tags WHERE name = ?", tag.name)
        .execute(&pool).await?;
    Ok(())
}

pub async fn add_tag_api(tag: Json<ModifyTags>) -> Result<(),MyError> {
    let pool = POOL.clone();
    sqlx::query!(
        "INSERT INTO tags VALUES (0,?)", tag.name)
        .execute(&pool).await?;
    Ok(())
}

pub async fn bind_tag_api(tag: Json<ModifyProductToTag>) -> Result<(), MyError> {
    let pool = POOL.clone();
    sqlx::query!(
        "INSERT INTO productsTotags(productID,tagID) SELECT ?, id FROM tags WHERE name = ?", tag.product_id, tag.name)
        .execute(&pool).await?;
    Ok(())
}

pub async fn unbind_tag_api(tag:Json<ModifyProductToTag>) -> Result<(), MyError> {
    let pool = POOL.clone();
    sqlx::query!(
        "DELETE FROM productsTotags where productID = ? AND tagID IN (SELECT id FROM tags WHERE name = ?)", tag.product_id, tag.name)
        .execute(&pool).await?;
    Ok(())
}

pub async fn get_popular_tags_api() -> Result<Json<Vec<PopularTag>>, MyError> {
    let pool = POOL.clone();
    let tags = sqlx::query_as!(PopularTag,
        "SELECT t.name AS tag_name, COUNT(pt.tagID) AS count
         FROM productsTotags pt
         LEFT JOIN tags t ON t.id  = pt.tagID 
         GROUP BY pt.tagID;")
        .fetch_all(&pool).await?;
    Ok(Json(tags))
}


