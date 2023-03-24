use std::fmt::Debug;

use futures::FutureExt;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::{html, Component, Context, Html};

use crate::util::{common::gql_uri, constant::ObjectId};

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_users.graphql",
    response_derives = "Debug"
)]
struct AllUsers;

pub struct Users {
    list: Vec<Value>,
}

pub enum Msg {
    UpdateList(Vec<Value>),
}

impl Component for Users {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { list: Vec::new() }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let res = fetch_users();
            ctx.link().send_future(res.map(Msg::UpdateList));
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateList(res) => {
                self.list = res;
                true
            },
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let users = self.list.iter().map(|user| {
            html! {
                <div>
                    <li>
                        <strong>
                            { &user["username"].as_str().unwrap() }
                            { " - length: " }
                            { &user["username"].as_str().unwrap().len() }
                        </strong>
                    </li>
                    <ul>
                        <li>{ &user["id"].as_str().unwrap() }</li>
                        <li>{ &user["email"].as_str().unwrap() }</li>
                    </ul>
                </div>
            }
        });

        html! {
            <>
                <h1>{ "all users" }</h1>
                <ul>
                    { for users }
                </ul>
            </>
        }
    }
}

async fn fetch_users() -> Vec<Value> {
    let token = "dLmqXwfi6hX3qqaHwadM5SD6VK9FRCdp";
    let build_query = AllUsers::build_query(all_users::Variables { token: token.to_string() });
    let query = serde_json::json!(build_query);

    let mut req_opts = RequestInit::new();
    req_opts
        .method("POST")
        .body(Some(&JsValue::from_str(&query.to_string())))
        .mode(RequestMode::Cors); // 可以不写，默认为 Cors

    let request =
        Request::new_with_str_and_init(&gql_uri(), &req_opts).expect("Build request failed");

    let window = web_sys::window().unwrap();
    let resp_value =
        JsFuture::from(window.fetch_with_request(&request)).await.expect("Response into js failed");
    let resp: Response = resp_value.dyn_into().unwrap();
    let resp_text = JsFuture::from(resp.text().expect("Response text failed"))
        .await
        .expect("Response text to js failed");

    let users_str = resp_text.as_string().unwrap();
    let users_value: Value = serde_json::from_str(&users_str).unwrap();
    let users_vec = users_value["data"]["allUsers"].as_array().unwrap().to_owned();

    users_vec
}
