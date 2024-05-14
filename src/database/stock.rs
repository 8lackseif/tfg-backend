
use rocket::serde::{json::Json, Deserialize, Serialize};
use sqlx::types::chrono::Local;

use super::{MyError, POOL};


#[derive(Debug,FromForm,Deserialize)]
pub struct StockVar {
    pub product_id:i32,
    pub quantity: i32
}

#[derive(Debug,FromForm,Deserialize)]
pub struct AddStocks {
    pub token: String,
    pub stocks: Vec<StockVar>
}

#[derive(Debug, Serialize)]
pub struct StockDto {
    product_id: i32,
    product_name: Option<String>,
    //date: NaiveDate,
    quantity: i32
}



pub async fn add_stocks(stocks: &Vec<StockVar>) -> Result<(),MyError> {
    let pool = POOL.clone();
    let curdate = Local::now();
    for s in stocks  {
        sqlx::query!(
            "INSERT INTO stockVar VALUES (0,?,?,?)", s.product_id, curdate, s.quantity)
            .execute(&pool).await?;
    }

    Ok(())
}

pub async fn change_stocks(stocks: &Vec<StockVar>) -> Result<(),MyError> {
    let pool = POOL.clone();
    for s in stocks {
        sqlx::query!(
            "UPDATE products SET stock = stock + ? WHERE id = ?", s.quantity, s.product_id)
            .execute(&pool).await?;
    }
   
    Ok(())
}

pub async fn get_stocks_api() -> Result<Json<Vec<StockDto>>, MyError> {
    let pool = POOL.clone();
    let stocks = sqlx::query_as!(StockDto,
        "SELECT s.productID as product_id, p.name as product_name, s.quantity as quantity 
        FROM stockVar s 
        LEFT JOIN products p ON s.productID = p.id
        LIMIT 15"
    ).fetch_all(&pool)
    .await?;

    Ok(Json(stocks))
}


