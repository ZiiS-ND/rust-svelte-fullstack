mod app;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let app = app::create_app().await;
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .expect("failed to start server");

    Ok(())
}
