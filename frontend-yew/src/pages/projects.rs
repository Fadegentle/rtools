use std::fmt::Debug;

use futures::FutureExt;
use gloo_net::http::Request;
use graphql_client::GraphQLQuery;
use serde_json::Value;
use yew::prelude::*;

use crate::util::{common::gql_uri, constant::ObjectId};

////////////////////////////////////////////////////
// Fetch projects data use `yew::services::fetch` //
////////////////////////////////////////////////////

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_projects.graphql",
    response_derives = "Debug"
)]
struct AllProjects;

#[derive(Debug)]
pub enum Msg {
    PassRequest,
    ReceiveResponse(Vec<Value>),
}

#[derive(Debug)]
pub struct Projects {
    list: Option<Vec<Value>>,
    error: Option<String>,
}

impl Component for Projects {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { list: None, error: None }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::PassRequest);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PassRequest => {
                let projects = fetch_projects();
                ctx.link().send_future(projects.map(Msg::ReceiveResponse));
                true
            },
            Msg::ReceiveResponse(data) => {
                if data.len() == 0 {
                    self.error = Some("No data.".to_string());
                } else {
                    self.list = Some(data);
                }

                // redraw so that the page displays projects data
                true
            },
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        ctx.link().send_message(Msg::PassRequest);
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{ "all projects" }</h1>

                { self.view_fetching() }
                { self.view_data() }
                { self.view_error() }
            </>
        }
    }
}

async fn fetch_projects() -> Vec<Value> {
    let all_projects = AllProjects::build_query(all_projects::Variables {});
    let query = serde_json::to_string(&all_projects).expect("Query failed to serialize.");
    let request = Request::post(&gql_uri()).body(query);
    let response = request.send().await.expect("Could not send request.");
    let projects_value = Value::from(response.text().await.expect("Could not get response text."));
    projects_value["data"]["allProjects"].as_array().unwrap().to_owned()
}

impl Projects {
    fn view_fetching(&self) -> Html {
        html! { <p></p> }
    }

    fn view_data(&self) -> Html {
        match self.list {
            Some(ref list) => {
                let projects = list.iter().map(|project| {
                    html! {
                        <div>
                            <li>
                                <strong>{ &project["subject"].as_str().unwrap() }</strong>
                            </li>
                            <ul>
                                <li>{ &project["userId"].as_str().unwrap() }</li>
                                <li>{ &project["id"].as_str().unwrap() }</li>
                                <li>
                                    <a href={ project["website"].as_str().unwrap().to_owned() }>
                                        { &project["website"].as_str().unwrap() }
                                    </a>
                                </li>
                            </ul>
                        </div>
                    }
                });

                html! {
                    <ul>
                        { for projects }
                    </ul>
                }
            },
            None => {
                html! {
                     <p>
                        { "No data." }
                     </p>
                }
            },
        }
    }

    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}
