use crate::models::user_models::{User, CreateUser, UpdateUser};
use sqlx::PgPool;
use anyhow::Result;

pub struct UserRepository;

impl UserRepository {
    pub async fn create(pool: &PgPool, user_data: CreateUser) -> Result<User> {
         //TODO Need to create validation before INSERT in DB
         
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username)
            VALUES ($1)
            RETURNING id, username
            "#,
            user_data.username,
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    // pub async fn find_by_id(pool: &PgPool, user_id: i32) -> Result<Option<User>> {
    //     let user = sqlx::query_as!(
    //         User,
    //         r#"
    //         SELECT id, username
    //         FROM users
    //         WHERE id = $1
    //         "#,
    //         user_id
    //     )
    //     .fetch_optional(pool)
    //     .await?;

    //     Ok(user)
    // }

    // pub async fn update(pool: &PgPool, user_id: i32, user_data: UpdateUser) -> Result<User> {
    //     let mut user = UserRepository::find_by_id(pool, user_id)
    //         .await?
    //         .ok_or(anyhow::anyhow!("User not found"))?;

    //     if let Some(username) = user_data.username {
    //         user.username = username;
    //     }

    //     if let Some(email) = user_data.email {
    //         user.email = email;
    //     }

    //     if let Some(password) = user_data.password {
    //         user.password_hash = hash_password(&password)?;
    //     }

    //     let updated_user = sqlx::query_as!(
    //         User,
    //         r#"
    //         UPDATE users
    //         SET 
    //             username = $1
    //         WHERE id = $2
    //         RETURNING id, username
    //         "#,
    //         user.username,
    //         user_id
    //     )
    //     .fetch_one(pool)
    //     .await?;

    //     Ok(updated_user)
    // }

    // pub async fn delete(pool: &PgPool, user_id: i32) -> Result<()> {
    //     sqlx::query!(
    //         "DELETE FROM users WHERE id = $1",
    //         user_id
    //     )
    //     .execute(pool)
    //     .await?;

    //     Ok(())
    // }
}

fn hash_password(password: &str) -> Result<String> {
    // В реальном приложении используйте argon2 или bcrypt
    Ok(password.to_string()) // Замените на реальное хэширование
}