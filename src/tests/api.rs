use std::net::SocketAddr;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::Router,
};
use http_body_util::BodyExt;
use rstest::rstest;
use tower::ServiceExt;

use crate::tests::fixtures::*;

#[rstest]
#[tokio::test]
async fn test_healthcheck(#[future] app: Router) {
    let response = app
        .await
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.into_body().collect().await.unwrap().to_bytes(),
        "¯\\_(ツ)_/¯"
    );
}

#[rstest]
#[tokio::test]
async fn test_healthcheck_real(#[future] live_app: SocketAddr) {
    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build_http();

    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{}/health", live_app.await))
                .header("Host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.into_body().collect().await.unwrap().to_bytes(),
        "¯\\_(ツ)_/¯"
    );
}
