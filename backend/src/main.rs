use actix_web::cookie::time::Instant;
use backend::{
    configs::{Configs, LogConfig},
    Application,
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let instant = Instant::now();

    // 初始化配置
    let confs = Configs::init_config()?;

    // 初始日志
    LogConfig::init(&confs.log)?;

    // 初始化服务器
    let application = Application::build(confs).await?;
    log::info!("🎉Started Application in {:.3?}", instant.elapsed());

    // 启动服务器
    application.run().await?;
    Ok(())
}
