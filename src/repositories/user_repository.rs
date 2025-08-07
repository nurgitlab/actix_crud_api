use crate::{errors::UserError, models::user_models::{CreateUser, UpdateUser, User}};
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

    pub async fn find_by_id(pool: &PgPool, user_id: i32) -> Result<User, UserError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Database error when fetching user {}: {}", user_id, e);
            UserError::DatabaseError(format!("Failed to fetch user {}", user_id))
        })?;
        
        user.ok_or_else(|| {
            log::info!("User {} not found", user_id);
            UserError::UserNotFound
        })
    }

    pub async fn update(pool: &PgPool, user_id: i32, user_data: UpdateUser) -> Result<User> {
        // let mut user = UserRepository::find_by_id(pool, user_id)
        //     .await?
        //     .ok_or(UserError::UserNotFound)?;

        // if let Some(username) = user_data.username {
        //     user.username = username;
        // }

        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET 
                username = $1
            WHERE id = $2
            RETURNING id, username
            "#,
            user_data.username,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_user)
    }

    pub async fn delete(pool: &PgPool, user_id: i32) -> Result<()> {
            // Сначала проверяем существование пользователя
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        if !exists {
            return Err(anyhow::anyhow!("User with id {} not found", user_id));
        }

        // Если пользователь существует, выполняем удаление
        let rows_affected = sqlx::query!(
            "DELETE FROM users WHERE id = $1",
            user_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        // Дополнительная проверка (на случай race condition)
        if rows_affected == 0 {
            Err(anyhow::anyhow!("User with id {} was not deleted", user_id))
        } else {
            Ok(())
        }
    }
}

fn hash_password(password: &str) -> Result<String> {
    // В реальном приложении используйте argon2 или bcrypt
    Ok(password.to_string()) // Замените на реальное хэширование
}