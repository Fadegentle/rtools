use async_graphql::{Context, ErrorExtensions, Object};
use validator::Validate;

use super::GqlResult;
use crate::{
    common::error::AppError,
    users::{
        models::{NewUser, User},
        services::Service,
    },
    State,
};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // 插入新用户
    async fn new_user(&self, ctx: &Context<'_>, mut new_user: NewUser) -> GqlResult<User> {
        // 参数校验
        new_user.validate().map_err(AppError::RequestParameterError.validation_extend())?;

        let pool = State::get_pool(ctx)?;
        let crypto = State::get_crypto_server(ctx)?;

        // 处理为 小写
        new_user.username.make_ascii_lowercase();
        new_user.email.make_ascii_lowercase();

        // 检查用户名重复
        let exists = User::exists_by_username(&pool, &new_user.username).await?;
        if exists {
            return Err(AppError::UsernameAlreadyExists.extend());
        }

        // 检查邮箱重复
        let exists = User::exists_by_email(&pool, &new_user.email).await?;
        if exists {
            return Err(AppError::EmailAlreadyExists.extend());
        }

        // 密码哈希
        let password_hash = crypto.generate_password_hash(&new_user.password).await?;

        let user = User::register_user(&pool, &new_user, &password_hash).await?;
        Ok(user)
    }
}
