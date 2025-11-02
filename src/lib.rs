mod env;
mod routes;
mod serializers;
mod state;
mod utils;
mod entities;
mod entities_helper;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::{catch_panic::CatchPanicLayer, compression::CompressionLayer};

use crate::{state::AppState, utils::middlewares::panic::handle_panic};

pub async fn run() {
    let app_state: AppState = AppState::new().await;
    let router = Router::new()
        .merge(routes::health::health_check_router())
        .merge(routes::auth::auth_router())
        .merge(routes::users::users_router())
        .merge(routes::categories::category_router())
        .merge(routes::retreats::retreat_router())
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(CompressionLayer::new())
        .with_state(app_state);

    let server_host = &env::ENV.server_host;
    let server_port = &env::ENV.server_port;
    let server_address: String = format!("{}:{}", server_host, server_port);

    let listener: TcpListener = TcpListener::bind(server_address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
