use super::POOL;

#[derive(Debug)]
pub struct User {
    id : i32,
    username : String,
    pwd : String
}

pub async fn query_users(){
    let pool = POOL.clone();
    let users = sqlx::query_as!(User,
        "SELECT * FROM users").fetch_all(&pool).await.unwrap();

    println!("{:?}",users);
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