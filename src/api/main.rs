#![allow(dead_code)]
use axum::{routing::get, Router};

#[tokio::main]

async fn main() {
    // build our application with a single route
    let app = Router::new().route("/ok", get(ok));

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ok() -> String {
    "ok".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ok_endpoint() {
        let response = ok().await;
        assert_eq!(response, "ok");
    }
}
