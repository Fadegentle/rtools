pub(crate) fn init() {
    log_init();
}

fn log_init() {
    if let Err(e) = log4rs::init_file("log/log4rs.yaml", Default::default()) {
        error!("日志初始化失败{}", e);
    }
    info!("日志初始化成功");
}
