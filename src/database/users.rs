use rocket::serde::{json::Json, Deserialize};
use super::{MyError, POOL};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

#[derive(Debug, FromForm, Deserialize)]
pub struct ResetPwd {
    username: String,
    pwd: String,
    pub token: String
}

#[derive(Debug)]
pub struct UserLog {
    pwd : String,
    rol : String,
    first_login: i8
}

#[derive(Debug,FromForm,Deserialize)]
pub struct UserData {
    pub username: String,
    pub pwd:String,
    pub rol: Option<String>,
    pub token:Option<String>
}

pub async fn register_user(data: Json<UserData>)-> Result<(), MyError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(data.pwd.as_bytes(), &salt).unwrap().to_string();
    let pool = POOL.clone();
    sqlx::query!(
        "INSERT INTO users values(0,?,?,?,1)",data.username,password_hash,data.rol)
        .execute(&pool).await?;        
    Ok(())
}

pub async fn login_user(data: Json<UserData>) -> Result<String, MyError> {
    let user: UserLog = get_user(&data.username).await?;
    let parsed_hash = PasswordHash::new(&user.pwd).unwrap();
    let argon2 = Argon2::default();
    if argon2.verify_password(data.pwd.as_bytes(), &parsed_hash).is_ok() {
        let key: Hmac<Sha256> = Hmac::new_from_slice(dotenv::var("SECRET").expect("failed to find SECRET on env").as_bytes()).unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("username", &data.username);
        claims.insert("role", &user.rol);
        let first_login = user.first_login.to_string();
        claims.insert("first_login", &first_login);
        let token_str = claims.sign_with_key(&key).unwrap();
        Ok(token_str)
    }
    else{
        Err(MyError::LoginError(format!("username or password incorrect")))
    }
}

pub async fn get_user(username: &str) -> Result<UserLog,MyError> {
    let pool = POOL.clone();
    let user = sqlx::query_as!(UserLog,
        "SELECT pwd, rol, first_login FROM users WHERE username=?",username)
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

pub async fn reset_password_api(data: Json<ResetPwd>) -> Result<(), MyError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(data.pwd.as_bytes(), &salt).unwrap().to_string();
    let pool = POOL.clone();
    sqlx::query!(
        "UPDATE users SET first_login = 0, pwd = ? WHERE username = ?",password_hash, data.username)
        .execute(&pool).await?;        
    Ok(())
}