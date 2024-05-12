use rocket::{serde::{json::Json, Deserialize, Serialize}};
use sqlx::types::chrono::Local;

use super::{MyError, POOL};


#[derive(Debug,FromForm,Deserialize)]
pub struct AddStockVar {
    pub product_id:i32,
    pub quantity: i32,
    pub token: String
}

pub async fn add_stock_var(stock_var: &Json<AddStockVar>) -> Result<(),MyError> {
    let pool = POOL.clone();
    let curdate = Local::now();
    sqlx::query!(
        "INSERT INTO stockVar VALUES (0,?,?,?)", stock_var.product_id, curdate, stock_var.quantity)
        .execute(&pool).await?;
    Ok(())
}

pub async fn change_stock(stock_var: &Json<AddStockVar>) -> Result<(),MyError> {
    let pool = POOL.clone();
    sqlx::query!(
        "UPDATE products SET stock = stock + ? WHERE id = ?", stock_var.product_id, stock_var.quantity)
        .execute(&pool).await?;
    Ok(())
}


