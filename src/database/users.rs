use rocket::serde::Deserialize;
use super::{MyError, POOL};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use hmac::{Hmac, Mac};
use jwt::{claims, SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;



#[derive(Debug)]
pub struct User {
    id : i32,
    username : String,
    pwd : String,
    rol : String
}

#[derive(Debug,FromForm,Deserialize)]
pub struct UserData {
    pub username: String,
    pub pwd:String,
    pub token:Option<String>
}

pub async fn register_user(username: &str, pwd: &str)-> Result<(), MyError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(pwd.as_bytes(), &salt).unwrap().to_string();
    let pool = POOL.clone();
    sqlx::query!(
        "INSERT INTO users values(0,?,?,'user')",username,password_hash)
        .execute(&pool).await?;        
    
    Ok(())
}

pub async fn login_user(username: &str, pwd: &str) -> Result<String, MyError> {
    let user: User = get_user(username).await?;
    let parsed_hash = PasswordHash::new(&user.pwd).unwrap();
    let argon2 = Argon2::default();
    if argon2.verify_password(pwd.as_bytes(), &parsed_hash).is_ok() {
        let key: Hmac<Sha256> = Hmac::new_from_slice(dotenv::var("SECRET").expect("failed to find SECRET on env").as_bytes()).unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("username", username);
        claims.insert("role", &user.rol);
        let token_str = claims.sign_with_key(&key).unwrap();
        Ok(token_str)
    }
    else{
        Err(MyError::LoginError(format!("username or password incorrect")))
    }
}

async fn get_user(username: &str) -> Result<User,MyError> {
    let pool = POOL.clone();
    let user = sqlx::query_as!(User,
        "SELECT * FROM users WHERE username=?",username)
        .fetch_optional(&pool).await?;
    
    if let Some (u) = user {
        Ok(u)
    }
    else{
        Err(MyError::UserNotFoundError(format!("user {} not found",username)))
    }   
}

pub async fn check(token: &str) -> Result<String, MyError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(dotenv::var("SECRET").expect("failed to find SECRET on env").as_bytes()).unwrap();
    let claims : BTreeMap<String, String> = token.verify_with_key(&key)?;
    Ok(claims["role"].to_string())
}

pub async fn register_admin(username: &str, pwd: &str)-> Result<(), MyError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(pwd.as_bytes(), &salt).unwrap().to_string();
    let pool = POOL.clone();
    sqlx::query!(
        "INSERT INTO users values(0,?,?,'administrator')",username,password_hash)
        .execute(&pool).await?;        
    
    Ok(())
}