#[macro_use]
extern crate log;
extern crate lazy_static;
extern crate log4rs;
extern crate rbatis;
extern crate rbdc;

mod app;
mod dbs;
mod gql;
mod pre;
mod users;
mod util;

use crate::gql::{build_schema, graphiql, graphql};
use actix_web::{guard, web, App, HttpServer};
use util::constant::CFG;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = build_schema().await;

    // TODO: init env into `pre.rs`
    let localhost = CFG.get("ADDRESS").unwrap();
    let port = CFG.get("PORT").unwrap();
    let localhost_port = format!("{}:{}", localhost, port);

    let gql_ver = CFG.get("GQL_VER").unwrap();
    let giql_ver = CFG.get("GIQL_VER").unwrap();

    println!("GraphQL UI: http://{}", localhost_port);

    HttpServer::new(move || {
        App::new()
            .app_data(schema.clone())
            .service(web::resource(gql_ver).guard(guard::Post()).to(graphql))
            .service(web::resource(giql_ver).guard(guard::Get()).to(graphiql))
    })
    .bind(localhost_port)?
    .run()
    .await
}
