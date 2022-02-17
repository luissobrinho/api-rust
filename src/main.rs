use crate::route::routes;

mod lib;
mod route;

#[tokio::main]
async fn main() {
    routes().await
}