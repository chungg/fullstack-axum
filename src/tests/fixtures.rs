use std::net::SocketAddr;

use axum::routing::Router;
use rstest::fixture;
use tokio::net::TcpListener;

use crate::app as app_def;

#[fixture]
pub fn app() -> Router {
    app_def()
}

#[fixture]
pub async fn live_app() -> SocketAddr {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app()).await.unwrap();
    });

    addr
}
