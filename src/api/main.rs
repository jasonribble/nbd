#![allow(dead_code)]
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ok", get(ok));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    println!("API: listening on {}", listener.local_addr().unwrap());
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
