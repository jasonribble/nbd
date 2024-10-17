#[cfg(test)]
mod tests {
    use reqwest;

    #[tokio::test]
    async fn test_ok_endpoint() {
        let url = "http://localhost:8080/ok";
        let response = reqwest::get(url).await.expect("Failed to send request");

        assert_eq!(response.status(), 200);

        let body = response.text().await.expect("Failed to get response text");
        assert_eq!(body, "ok");
    }
}
