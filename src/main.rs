#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::{Response, Responder};
use rocket::Request;
use rocket::http::{Status, ContentType};
use std::io::Cursor;

#[derive(Debug, Clone, PartialEq)]
struct Content{
    message : String,  
}

impl Responder<'static> for Content {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        Response::build()
            .header(ContentType::Plain)
            .sized_body(Cursor::new(self.message))
            .raw_header("Access-Control-Allow-Origin", "*")
            .ok()
    }
}


#[get("/login")]
fn login() -> Content {
    
    Content{
        message:"hola".to_string()
    }
}
fn main() {
    rocket::ignite().mount("/", routes![login]).launch();
}

