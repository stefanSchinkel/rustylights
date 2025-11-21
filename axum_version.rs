// src/main.rs

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::fs;

// --- Models ---

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub name: String,
    pub on: u32,
    pub off: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Binaries {
    pub send: String,
    pub codesend: String,
    pub sniff: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub binaries: Binaries,
    pub devices: Vec<Device>,
}

// --- Message Response ---

#[derive(Serialize, Debug)]
pub struct Message {
    message: String,
}

// --- App State ---

pub struct AppState {
    pub config: Arc<Config>,
}

// --- Handlers ---

// GET /
async fn index() -> &'static str {
    "Hello, world!"
}

// GET /devices
async fn get_devices(State(state): State<AppState>) -> impl IntoResponse {
    let message = Message {
        message: format!("Found {} devices", state.config.devices.len()),
    };
    // Return JSON response
    axum::Json(message)
}

// --- Application Setup ---

async fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("./config.json").await?;
    let config: Config = serde_json::from_str(&data)?;
    Ok(config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config
    let config = load_config().await?;

    // Wrap in Arc for sharing across handlers
    let app_state = AppState {
        config: Arc::new(config),
    };

    // Build router
    let app = Router::new()
        .route("/", get(index))
        .route("/devices", get(get_devices))
        .with_state(app_state);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("🚀 Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
