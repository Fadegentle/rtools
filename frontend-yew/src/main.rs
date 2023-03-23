#![recursion_limit = "1024"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod pages;
mod util;

use console_error_panic_hook::set_once as set_panic_hook;
use pages::{home::Home, projects::Projects, users::Users};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/users")]
    Users,
    #[at("/projects")]
    Projects,
    #[at("/")]
    Home,
}

fn switch(switch: Route) -> Html {
    match switch {
        Route::Users => {
            html! { <Users /> }
        },
        Route::Projects => {
            html! { <Projects /> }
        },
        Route::Home => {
            html! { <Home /> }
        },
    }
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div class="logo-title">
                <img src="imgs/budshome.png" />
                <strong>{ "frontend-yew / tide-async-graphql-mongodb" }</strong>
            </div>
            <div class="nav">
                <Link<Route> to={Route::Users}>{ "用户列表" }</Link<Route>>
                <span class="placeholder">{ " - " }</span>
                <Link<Route> to={Route::Projects}>{ "项目列表" }</Link<Route>>
                <span class="placeholder">{ " - " }</span>
                <Link<Route> to={Route::Home}>{ "主页" }</Link<Route>>
            </div>
            <main>
                <Switch<Route> render={switch} />
            </main>
            </>
        }
    }
}

fn main() {
    set_panic_hook();
    yew::Renderer::<App>::new().render();
}
