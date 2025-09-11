use axum::{Router, routing::post};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tracing_subscriber;

mod pages;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let routes = generate_route_list(pages::app);

    let app = Router::new()
        .merge(LeptosRoutes::new(routes, pages::app))
        .route("/api/login", post(pages::api_login));

    let addr = "0.0.0.0:3000".parse().unwrap();
    tracing::info!("Server running at http://{}/", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
