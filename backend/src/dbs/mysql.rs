use rbatis::rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;

use crate::util::constant::CFG;

pub async fn my_pool() -> Rbatis {
    let rb = Rbatis::new();
    rb.init(MysqlDriver {}, CFG.get("MYSQL_URI").unwrap());
    rb
}
