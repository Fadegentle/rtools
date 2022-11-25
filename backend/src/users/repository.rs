use anyhow::{Context, Result};
use sqlx::PgPool;

use super::models::{NewUser, User};

pub async fn create(pool: &PgPool, new_user: &NewUser, password_hash: &str) -> Result<User> {
    let row = sqlx::query_as!(
            User,
            "INSERT INTO users(username, nickname, email, password_hash) VALUES ($1, $2, $3, $4) RETURNING *",
            &new_user.username,
            &new_user.nickname,
            &new_user.email,
            password_hash
        )
            .fetch_one(pool)
            .await
            .context("创建用户")?;

    Ok(row)
}

/// 根据用户名列出所有用户
pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>> {
    let row = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(pool)
        .await
        .context("列出所有用户")?;

    Ok(row)
}

/// 根据用户名查询用户
pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
    let row = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
        .fetch_optional(pool)
        .await
        .context("查询用户")?;

    Ok(row)
}

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
    let row = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(pool)
        .await
        .context("查询用户")?;

    Ok(row)
}

/// 根据用户名查询用户2
pub async fn find_by_username2(pool: &PgPool, username: &str) -> Result<User> {
    let row = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
        .fetch_one(pool)
        .await
        .context("根据用户名查询用户2")?;

    Ok(row)
}

/// 检查用户是否存在
pub async fn exists_by_username(pool: &PgPool, username: &str) -> Result<bool> {
    let row = sqlx::query!("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)", username,)
        .fetch_one(pool)
        .await
        .context("检查用户是否存在")?;
    let exists: Option<bool> = row.exists;
    Ok(exists.unwrap_or_default())
}

pub async fn exists_by_email(pool: &PgPool, email: &str) -> Result<bool> {
    let row = sqlx::query!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)", email,)
        .fetch_one(pool)
        .await
        .context("检查邮箱是否存在")?;
    let exists: Option<bool> = row.exists;
    Ok(exists.unwrap_or_default())
}
