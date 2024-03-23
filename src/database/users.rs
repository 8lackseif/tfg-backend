use rocket::{serde::Deserialize};
use super::{MyError, POOL};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};



#[derive(Debug)]
pub struct User {
    id : i32,
    username : String,
    pwd : String
}

#[derive(Debug,FromForm,Deserialize)]
pub struct UserData {
    pub username: String,
    pub pwd:String,
}

pub async fn query_users(){
    let pool = POOL.clone();
    let users = sqlx::query_as!(User,
        "SELECT * FROM users").fetch_all(&pool).await.unwrap();

    println!("{:?}",users);
}

pub async fn register_user(username: &str, pwd: &str)-> Result<(), MyError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(pwd.as_bytes(), &salt).unwrap().to_string();
    let pool = POOL.clone();
    sqlx::query!(
        "INSERT INTO users values(0,?,?)",username,password_hash,)
        .execute(&pool).await?;        
    
    Ok(())
}

pub async fn get_user(username: &str){
    let pool = POOL.clone();
    let user = sqlx::query_as!(User,
        "SELECT * FROM users WHERE username=?",username)
        .fetch_optional(&pool).await.unwrap();

    if let Some(user) = user {
        println!("usuario encontrado: {}", user.username);
    }
    else{
        println!("usuario {} no encontrado.", username);
    }
    
}