#[macro_use]
extern crate log;
extern crate lazy_static;
extern crate log4rs;

use std::sync::Arc;

use actix_web::{
    dev::Server,
    guard::{Get, Post},
    middleware::Logger,
    web::{resource, ServiceConfig},
    App, HttpServer,
};
use anyhow::Context;
use async_graphql::Context as GqlContext;
use configs::Configs;
use gql::{graphiql, graphql, health_check, ServiceSchema};
use regex::Regex;
use security::crypto::CryptoService;
use sqlx::{PgPool, Pool, Postgres};

use crate::{
    configs::{CryptoConfig, DatabaseConfig},
    gql::GqlResult,
};

mod app;
mod common;
pub mod configs;
mod gql;
mod pre;
mod security;
mod users;
mod util;

lazy_static::lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"(@)").unwrap();
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]{4,16}$").unwrap();
}

/// 全局的 state
pub struct State {
    // 数据库连接池
    pool: Arc<PgPool>,
    // 加密服务
    crypto: Arc<CryptoService>,
}

impl State {
    // 通过 GqlContext 获取 数据库连接池
    pub fn get_pool(ctx: &GqlContext<'_>) -> GqlResult<Arc<Pool<Postgres>>> {
        Ok(ctx.data::<Arc<State>>()?.pool.clone())
    }

    // 通过 GqlContext 获取 加密服务
    pub fn get_crypto_server(ctx: &GqlContext<'_>) -> GqlResult<Arc<CryptoService>> {
        Ok(ctx.data::<Arc<State>>()?.crypto.clone())
    }
}

/// http server application
pub struct Application {
    server: Server,
}

impl Application {
    /// 构建 服务器
    pub async fn build(configs: Arc<Configs>) -> anyhow::Result<Application> {
        // 初始化静态常量
        lazy_static::initialize(&EMAIL_REGEX);
        lazy_static::initialize(&USERNAME_REGEX);
        log::info!("初始化 '静态常量' 完成");

        // 链接数据库
        let pool = DatabaseConfig::init(&configs.database).await?;
        let crypto = CryptoConfig::get_crypto_server(&configs.crypto);
        let state = Arc::new(State { pool, crypto });

        // 初始化 GraphQL schema.
        let schema = gql::build_schema(state.clone(), &configs.graphql).await;
        log::info!(r#"初始化 'GraphQL Schema' 完成! "#);

        let address = configs.server.get_address();
        let enable = &configs.graphql.graphiql.enable;
        if enable.unwrap_or(false) {
            log::info!("🚀GraphQL UI: http://{}{}", address, &configs.graphql.graphiql.path);
        }

        let server = build_actix_server(configs, address, state, schema)?;

        Ok(Application { server })
    }

    /// 启动
    pub async fn run(self) -> anyhow::Result<(), std::io::Error> {
        self.server.await
    }
}

/// 构建 服务器
fn build_actix_server(
    configs: Arc<Configs>,
    address: String,
    state: Arc<State>,
    schema: ServiceSchema,
) -> anyhow::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(configs.clone())
            .app_data(state.clone())
            .app_data(schema.clone())
            .configure(|cfg| register_service(cfg, configs.clone()))
    })
    .bind(address)
    .context("绑定监听地址失败")?
    .run();
    Ok(server)
}

/// 注册路由 每一个worker都会注册一下
fn register_service(cfg: &mut ServiceConfig, configs: Arc<Configs>) {
    let graphql_config = &configs.graphql;

    // graphql 入口
    cfg.service(resource(&graphql_config.path).guard(Post()).to(graphql));

    // rest 健康检查
    cfg.service(resource(configs.server.get_health_check()).guard(Get()).to(health_check));

    // 开发环境的工具
    let enable = graphql_config.graphiql.enable;
    if enable.unwrap_or(false) {
        cfg.service(resource(&graphql_config.graphiql.path).guard(Get()).to(graphiql));
    }
}

#[test]
fn test_email_regex() {
    lazy_static::initialize(&EMAIL_REGEX);
    let is_email = EMAIL_REGEX.is_match("text@test.com");
    assert!(is_email);
}

#[test]
fn test_username_regex() {
    lazy_static::initialize(&USERNAME_REGEX);
    let is_username = USERNAME_REGEX.is_match("liteng001");
    assert!(is_username);
}
