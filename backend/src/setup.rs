use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::{Pool, MySql};
use tracing::info;
 
pub async fn ensure_admin(db: &Pool<MySql>) {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM admins")
        .fetch_one(db)
        .await
        .expect("Failed to query users");

    if count > 0 {
        return;
    }
 
    info!("No admin user found — first time setup");
    
    let username = prompt("Enter admin username: ");
    
    let password = loop {
        let password = prompt("Enter admin password: ");
        let password_check = prompt("Enter admin password again: ");

        if password == password_check {
            break password;
        }

        println!("Passwords do not match, try again");
    };
 
    let hash = hash_password(&password);
 
    sqlx::query("INSERT INTO admins (username, password) VALUES (?, ?)")
        .bind(&username)
        .bind(&hash)
        .execute(db)
        .await
        .expect("Failed to create admin user");
 
    println!("Admin user '{}' created", username);
}
 
fn prompt(label: &str) -> String {
    use std::io::{self, Write};
    print!("{}", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
 
fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}
