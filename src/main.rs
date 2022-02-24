use crate::route::create_routes;

mod lib;
mod route;

#[tokio::main]
async fn main() {
    let routes = create_routes();
    warp::serve(routes).run(([127, 0, 0, 1], 80)).await
}
