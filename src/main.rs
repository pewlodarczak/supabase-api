use axum::{
    routing::get,
    Router,
    response::{Html, IntoResponse},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
//use std::path::Path;
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
}

// Serve the index.html page
async fn index() -> impl IntoResponse {
    const HTML_CONTENT: &str = include_str!("../index.html");
    Html(HTML_CONTENT).into_response()
}

async fn get_users() -> impl IntoResponse {
    let supabase_url = "https://vceberzqpcutgbhcwvaq.supabase.co/rest/v1/users";
    let supabase_api_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InZjZWJlcnpxcGN1dGdiaGN3dmFxIiwicm9sZSI6ImFub24iLCJpYXQiOjE3Mjc0NDAyMzksImV4cCI6MjA0MzAxNjIzOX0.zZHjTglFaKwYlsMkpDj3ScEz6_6mz2LN1WMxm8cAmgA";

    let client = Client::new();
    let res = client
        .get(supabase_url)
        .header("apikey", supabase_api_key)
        .header("Authorization", format!("Bearer {}", supabase_api_key))
        .header("Accept", "application/json")
        .send()
        .await;

    match res {
        Ok(resp) => {
            let users: Result<Vec<User>, _> = resp.json().await;
            match users {
                Ok(data) => axum::Json(data).into_response(),
                Err(_) => axum::Json("Failed to parse users").into_response(),
            }
        }
        Err(_) => axum::Json("Failed to call Supabase").into_response(),
    }
}

const INDEX_HTML_FALLBACK: &str = r#"<!DOCTYPE html><html><body><h1>Supabase API</h1><p>Visit <a href="/users">/users</a> for API</p></body></html>"#;

#[shuttle_runtime::main]
async fn shuttle_main() -> shuttle_axum::ShuttleAxum {
    let app = Router::new()
        .route("/", get(index))
        .route("/users", get(get_users));

    // Create a socket address with a fixed port
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    Ok(app.into())
}