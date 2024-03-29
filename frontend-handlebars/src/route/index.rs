use serde_json::json;
use tide::{self, Request, Server};

use crate::{
    route::{projects::project_index, users::user_index},
    util::common::Tpl,
    State,
};

// pub async fn push_res(mut app: Server<State>) -> Server<State> {
pub async fn push_res(app: &mut Server<State>) {
    app.at("/static").serve_dir("./static").unwrap();

    // environment variables defined in .env file
    app.at("/").get(index);
    app.at("users").get(user_index);
    app.at("projects").get(project_index);

    // app
}

async fn index(_req: Request<State>) -> tide::Result {
    let index: Tpl = Tpl::new("index").await;

    // make data and render it
    let data =
        json!({"app_name": "frontend-handlebars - tide-async-graphql-mongodb", "author": "zzy"});

    index.render(&data).await
}
