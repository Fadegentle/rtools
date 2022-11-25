use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::users::models::{NewUser, User};
#[async_trait]
pub trait Service {
    /// 注册用户
    async fn register_user(pool: &PgPool, new_user: &NewUser, password_hash: &str) -> Result<User>;

    /// 查询所有用户
    async fn get_all_users(pool: &PgPool) -> Result<Vec<User>>;

    /// 根据用户名查询用户
    async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<User>>;

    /// 根据邮箱查询查询用户
    async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>>;

    /// 根据用户名查询用户2
    async fn find_by_username2(pool: &PgPool, username: &str) -> Result<User>;

    /// 检查用户是否存在
    async fn exists_by_username(pool: &PgPool, username: &str) -> Result<bool>;

    /// 检查邮箱是否存在
    async fn exists_by_email(pool: &PgPool, email: &str) -> Result<bool>;
}

#[async_trait]
impl Service for User {
    async fn register_user(pool: &PgPool, new_user: &NewUser, password_hash: &str) -> Result<User> {
        super::repository::create(pool, new_user, password_hash).await
    }

    async fn get_all_users(pool: &PgPool) -> Result<Vec<User>> {
        super::repository::get_all_users(pool).await
    }

    async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
        super::repository::find_by_username(pool, username).await
    }

    async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
        super::repository::find_by_email(pool, email).await
    }

    async fn find_by_username2(pool: &PgPool, username: &str) -> Result<User> {
        super::repository::find_by_username2(pool, username).await
    }

    async fn exists_by_username(pool: &PgPool, username: &str) -> Result<bool> {
        super::repository::exists_by_username(pool, username).await
    }

    async fn exists_by_email(pool: &PgPool, email: &str) -> Result<bool> {
        super::repository::exists_by_email(pool, email).await
    }
}

// // 查询所有用户
// pub async fn get_all_users(pool: &PgPool) -> GqlResult<Vec<User>> {
//     let User = User::select_all(pool).await.unwrap();

//     if User.len() > 0 {
//         Ok(User)
//     } else {
//         Err(Error::new("1-all-User").extend_with(|_, e| e.set("details", "No records")))
//     }
// }

// rbatis::impl_select!(User{select_by_email(email:&str) -> Option => "`where email = #{email} limit
// 1`"}); // 通过 email 获取用户
// pub async fn get_user_by_email(pool: &PgPool, email: &str) -> GqlResult<User> {
//     User::select_by_email(&mut pool, email).await?.ok_or_else(|| {
//         Error::new("email 不存在").extend_with(|_, e| e.set("details", "1_EMAIL_NOT_EXIStS"))
//     })
// }

// // 插入新用户
// pub async fn new_user(pool: &PgPool, mut new_user: NewUser) -> GqlResult<User> {
//     new_user.email = new_user.email.to_lowercase();

//     if self::get_user_by_email(pool, &new_user.email).await.is_ok() {
//         Err(Error::new("email 已存在").extend_with(|_, e| e.set("details", "1_EMAIL_EXIStS")))
//     } else {
//         new_user.cred = "P38V7+1Q5sjuKvaZEXnXQqI9SiY6ZMisB8QfUOP91Ao=".to_string();
//         NewUser::insert(&mut pool, &new_user).await.expect("插入 user 数据时出错");

//         self::get_user_by_email(pool, &new_user.email).await
//     }
// }
