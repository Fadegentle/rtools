use handlebars::Handlebars;
use serde::Serialize;
use tide::{http::mime::HTML, Body, Response, StatusCode};

use crate::util::constant::CFG;

pub async fn gql_uri() -> String {
    let address = CFG.get("ADDRESS").unwrap();
    let gql_port = CFG.get("GRAPHQL_PORT").unwrap();
    let gql_path = CFG.get("GRAPHQL_PATH").unwrap();

    format!("http://{}:{}/{}", address, gql_port, gql_path)
}

pub async fn rhai_dir() -> String {
    format!("./{}/", "scripts")
}

pub struct Tpl<'tpl> {
    pub name: String,
    pub reg: Handlebars<'tpl>,
}

impl<'tpl> Tpl<'tpl> {
    pub async fn new(rel_path: &str) -> Tpl<'tpl> {
        let tpl_name = &rel_path.replace("/", "_");
        let abs_path = format!("./templates/{}.html", rel_path);

        // create the handlebars registry
        let mut hbs_reg = Handlebars::new();
        // enable dev mode for template reloading
        // hbs_reg.set_dev_mode(true);
        // register template from a file and assign a name to it
        hbs_reg.register_template_file(tpl_name, abs_path).unwrap();

        Tpl { name: tpl_name.to_string(), reg: hbs_reg }
    }

    pub async fn render<T>(&self, data: &T) -> tide::Result
    where
        T: Serialize,
    {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_content_type(HTML);
        resp.set_body(Body::from_string(self.reg.render(&self.name, data).unwrap()));

        Ok(resp.into())
    }
}
