use rocket::serde::{json::Json, Deserialize, Serialize};
use sqlx::types::{chrono::Local, time::Date, BigDecimal};

use super::{MyError, POOL};

#[derive(Debug, FromForm, Deserialize)]
pub struct StockVar {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, FromForm, Deserialize)]
pub struct AddStocks {
    pub token: String,
    pub stocks: Vec<StockVar>,
}

#[derive(Debug, FromForm, Deserialize)]
pub struct ProductStockHistory {
    pub token: String,
    pub id: i32,
}

#[derive(Debug, Serialize)]
pub struct StockDto {
    product_id: i32,
    product_name: Option<String>,
    #[serde(with = "my_date_format")]
    date: Date,
    quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct StockHistory {
    year: Option<i32>,
    #[serde(with = "my_big_decimal_format")]
    quantity: Option<BigDecimal>,
}

mod my_big_decimal_format {
    use bigdecimal::ToPrimitive;
    use serde::Serializer;
    use sqlx::types::BigDecimal;

    pub fn serialize<S>(number: &Option<BigDecimal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let x = number
            .as_ref()
            .map(|n| serializer.serialize_i32(n.to_i32().unwrap()).unwrap());
        Ok(x.unwrap())
    }
}

mod my_date_format {
    use serde::Serializer;
    use sqlx::types::time::Date;
    use time::macros::format_description;

    pub fn serialize<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = format_description!("[day]/[month]/[year]");
        let s = &date.format(&format).unwrap();
        serializer.serialize_str(&s)
    }
}

pub async fn add_stocks(stocks: &Vec<StockVar>) -> Result<(), MyError> {
    let pool = POOL.clone();
    let curdate = Local::now();
    for s in stocks {
        sqlx::query!(
            "INSERT INTO stockVar VALUES (0,?,?,?)",
            s.product_id,
            curdate,
            s.quantity
        )
        .execute(&pool)
        .await?;
    }

    Ok(())
}

pub async fn change_stocks(stocks: &Vec<StockVar>) -> Result<(), MyError> {
    let pool = POOL.clone();
    for s in stocks {
        sqlx::query!(
            "UPDATE products SET stock = stock + ? WHERE id = ?",
            s.quantity,
            s.product_id
        )
        .execute(&pool)
        .await?;
    }

    Ok(())
}

pub async fn get_stocks_api() -> Result<Json<Vec<StockDto>>, MyError> {
    let pool = POOL.clone();
    let stocks = sqlx::query_as!(StockDto,
        "SELECT s.productID as product_id, p.name as product_name, s.varDate as date, s.quantity as quantity 
        FROM stockVar s 
        LEFT JOIN products p ON s.productID = p.id
        ORDER BY s.varDate DESC, s.varID DESC
        LIMIT 15"
    ).fetch_all(&pool)
    .await?;

    Ok(Json(stocks))
}

pub async fn get_stock_history_api(product_id: i32) -> Result<Json<Vec<StockHistory>>, MyError> {
    let pool = POOL.clone();
    let stock_history = sqlx::query_as!(
        StockHistory,
        "SELECT YEAR(varDate) AS year, SUM(quantity) AS quantity
        FROM stockVar
        WHERE productID = ?
        GROUP BY YEAR(varDate)
        ORDER BY YEAR(varDate)",
        product_id
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(stock_history))
}

pub async fn add_stock_from_code(code: &str, quantity: i32) -> Result<(), MyError> {
    let pool = POOL.clone();
    let curdate = Local::now();
    sqlx::query!(
    "INSERT INTO stockVar(productID, varDate ,quantity) VALUES((SELECT id FROM products WHERE code=?),? ,?)", code, curdate, quantity)
    .execute(&pool).await?;
    Ok(())
}
