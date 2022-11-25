use async_graphql::{Context, Object};

use crate::{
    common::error::AppError,
    gql::GqlResult,
    users::{models::User, services::Service},
    State,
};

pub struct QueryRoot;

// TODO: pub type Storage = Arc<Mutex<Slab<Book>>>;

#[Object]
impl QueryRoot {
    // 获取所有用户
    async fn get_all_users(&self, ctx: &Context<'_>) -> GqlResult<Vec<User>> {
        let pool = State::get_pool(ctx)?;
        Ok(User::get_all_users(&pool).await.map_err(AppError::InternalError.log_extend())?)
    }

    // 根据 email 获取用户
    async fn find_by_email(&self, ctx: &Context<'_>, email: String) -> GqlResult<Option<User>> {
        let pool = State::get_pool(ctx)?;
        Ok(User::find_by_email(&pool, &email)
            .await
            .map_err(AppError::InternalError.log_extend())?)
    }
}
