mod route;
mod util;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    // Something in Tide State
    let app_state = State {};
    let mut app = tide::with_state(app_state);
    // app = push_res(app).await;
    route::index::push_res(&mut app).await;

    app.listen(format!("{}:{}", "127.0.0.1", "3000")).await?;

    Ok(())
}

//  Tide application scope state.
#[derive(Clone)]
pub struct State {}
