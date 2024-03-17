#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

mod database;

use database::users::{query_users, User};
use dotenv::dotenv;
// use rocket::response::{Response, Responder};
// use rocket::Request;
// use rocket::http::{Status, ContentType};
// use std::io::Cursor;

// use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};


#[derive(Debug, Clone, PartialEq)]
struct Content {
    message : String,  
}

// impl Responder<'static> for Content {
//     fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
//         Response::build()
//             .header(ContentType::Plain)
//             .sized_body(Cursor::new(self.message))
//             .raw_header("Access-Control-Allow-Origin", "*")
//             .ok()
//     }
// }


#[post("/login")]
async fn login() -> String {
    query_users().await;
    "hola".to_string()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allow_credentials(true);

    rocket::build().attach(cors.to_cors().unwrap()).mount("/", routes![login]).launch().await.unwrap();
}

