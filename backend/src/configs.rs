use std::{any::type_name, env::current_dir, path::PathBuf, sync::Arc, time::Duration};

use anyhow::Context;
use log::LevelFilter;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};

use crate::security::crypto::CryptoService;

/// 配置文件目录
pub const CONFIG_PATH: &str = "configs/";
pub const SERVER_CONFIG_PATH: &str = "server/configs/";

/// 配置文件默认文件
pub const DEFAULT_CONFIG: &str = "base";

/// 配置环境标识
pub const SERVER_ENVIRONMENT: &str = "SERVER_ENVIRONMENT";

/// 环境变量覆盖配置文件前缀
pub const SERVER_PREFIX: &str = "SERVER";

/// 环境变量覆盖配置文件分隔符
pub const SEPARATOR: &str = "_";

/// 默认健康检查地址
pub const HEALTH_CHECK: &str = "/health_check";

/// 配置项结构体
#[derive(Deserialize, Clone, Debug)]
pub struct Configs {
    pub server: ServerConfig,
    pub graphql: GraphQlConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
    pub crypto: CryptoConfig,
}

impl Configs {
    /// 初始化配置文件
    pub fn init_config() -> anyhow::Result<Arc<Configs>> {
        // 加载环境变量
        dotenv::dotenv().ok();

        // 读取当前环境标志
        let environment = dotenv::var(SERVER_ENVIRONMENT)
            .context(format!("读取当前环境标志:[{}] 失败!", SERVER_ENVIRONMENT))?;

        // 加载配置文件
        let config_dir = get_config_dir()?;
        let config = config::Config::builder()
            .add_source(config::File::from(config_dir.join(DEFAULT_CONFIG)))
            .add_source(config::File::from(config_dir.join(environment))) // 从环境变量或.env中添加设置（以APP前缀和'__'作为分隔符）
            .add_source(config::Environment::with_prefix(SERVER_PREFIX).separator(SEPARATOR)) // APP_SERVER_PORT = 5001，将覆盖 ApplicationConfig.server.port
            .build()?
            .try_deserialize()
            .context("配置文件转换错误!")?; // 将读取的配置文件转换为配置文件结构体

        Ok(Arc::new(config))
    }
}

/// 服务配置
#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub name: String,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub context_path: Option<String>,
    pub health_check: Option<String>,
}

impl ServerConfig {
    /// 获取服务地址
    pub fn get_address(&self) -> String {
        format!("{}:{}", &self.host, &self.port)
    }

    /// 获取健康检查地址
    pub fn get_health_check(&self) -> String {
        if let Some(path) = &self.health_check { path.clone() } else { String::from(HEALTH_CHECK) }
    }
}

/// Graphql配置
#[derive(Deserialize, Clone, Debug)]
pub struct GraphQlConfig {
    pub path: String,
    pub tracing: Option<bool>,
    pub graphiql: GraphiQlConfig,
}

/// Graphiql配置
#[derive(Deserialize, Clone, Debug)]
pub struct GraphiQlConfig {
    pub path: String,
    pub enable: Option<bool>,
}

/// 数据库配置
#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseConfig {
    /// 初始化数据库连接池
    pub async fn init(&self) -> anyhow::Result<Arc<Pool<Postgres>>> {
        let mut options = PgConnectOptions::new()
            .username(&self.username)
            .password(&self.password)
            .host(&self.host)
            .port(self.port)
            .database(&self.database_name);
        // 设置 sql 日志级别
        options.log_statements(LevelFilter::Debug);
        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(2))
            .connect_with(options)
            .await?;
        log::info!("初始化 '数据库' 完成");
        Ok(Arc::new(pool))
    }
}

/// 日志相关配置
#[derive(Deserialize, Clone, Debug)]
pub struct LogConfig {
    /// 日志配置文件
    pub file: String,
}

impl LogConfig {
    /// 初始化日志配置
    pub fn init(config: &LogConfig) -> anyhow::Result<()> {
        let config_dir = get_config_dir()?;
        let result = log4rs::init_file(config_dir.join(&config.file), Default::default())
            .context(format!("初始化日志配置:[{}]失败!", &config.file));
        log::info!(r#"初始化 '配置文件 日志' 完成!"#);
        result
    }
}

/// 加密服务相关配置
#[derive(Deserialize, Clone, Debug)]
pub struct CryptoConfig {
    pub hash: HashConfig,
    pub jwt: JwtConfig,
}

/// 加密服务相关配置
#[derive(Deserialize, Clone, Debug)]
pub struct HashConfig {
    /// 密码盐
    pub salt: String,
    /// 秘钥
    pub secret: String,
}

/// jwt相关配置
#[derive(Deserialize, Clone, Debug)]
pub struct JwtConfig {
    /// 秘钥
    pub secret: String,
    /// 访问token过期时间
    #[serde(with = "humantime_serde", default)]
    pub access_expires: Option<Duration>,
    /// 刷新token过期时间
    #[serde(with = "humantime_serde", default)]
    pub refash_expires: Option<Duration>,
    /// 签发人
    #[serde(default = "default_issuer")]
    pub issuer: String,
}

/// 签发人默认值
fn default_issuer() -> String {
    "Server".to_string()
}

impl CryptoConfig {
    /// 获取加密服务
    pub fn get_crypto_server(&self) -> Arc<CryptoService> {
        let access_expires =
            self.jwt.access_expires.unwrap_or_else(|| Duration::from_secs(30 * 60));

        let refash_expires =
            self.jwt.refash_expires.unwrap_or_else(|| Duration::from_secs(30 * 60));

        let crypto = CryptoService {
            hash_salt: Arc::new(self.hash.salt.clone()),
            hash_secret: Arc::new(self.hash.secret.clone()),
            jwt_secret: Arc::new(self.jwt.secret.clone()),
            access_expires: Arc::new(chrono::Duration::from_std(access_expires).unwrap()),
            refash_expires: Arc::new(chrono::Duration::from_std(refash_expires).unwrap()),
            issuer: Arc::new(self.jwt.issuer.clone()),
        };
        log::info!("初始化 '加密服务:[{}]' 完成!", type_name::<CryptoService>());
        Arc::new(crypto)
    }
}

/// 获取配置文件路径
fn get_config_dir() -> anyhow::Result<PathBuf> {
    let base_path = current_dir().context("无法确定当前目录")?;

    let mut config_dir = base_path.join(CONFIG_PATH);

    if !config_dir.as_path().exists() {
        config_dir = base_path.join(SERVER_CONFIG_PATH);
    };
    Ok(config_dir)
}
