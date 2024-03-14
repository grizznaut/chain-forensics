use axum::{extract::Path, extract::State, routing::get, Json, Router};
use bitcoincore_rpc::Auth;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;

struct AppState {
    rpc_client: bitcoincore_rpc::Client,
    reqwest_client: reqwest::Client,
    db_pool: sqlx::MySqlPool,
}

#[derive(Serialize)]
struct Entity {
    name: Option<String>,
}

// Placeholder endpoint, for now just returns dummy data.
async fn entities(State(state): State<Arc<AppState>>) -> Json<Vec<Entity>> {
    let entities = sqlx::query_as!(Entity, "SELECT name FROM entity")
        .fetch_all(&state.db_pool)
        .await
        .unwrap();
    Json(entities)
}

#[derive(Serialize, Deserialize)]
struct AddrStats {
    funded_txo_count: u32,
    funded_txo_sum: u64,
    spent_txo_count: u32,
    spent_txo_sum: u64,
    tx_count: u32,
}

#[derive(Serialize, Deserialize)]
struct Address {
    address: String,
    chain_stats: AddrStats,
    mempool_stats: AddrStats,
}

async fn address(Path(address): Path<String>, State(state): State<Arc<AppState>>) -> Json<Address> {
    let url = format!("http://127.0.0.1:3002/address/{}", address);
    let res = state.reqwest_client.get(url).send().await.unwrap();
    let address_details = res.json().await.unwrap();
    Json(address_details)
}

async fn setup_database() -> sqlx::MySqlPool {
    // Create connection pool
    let db_pool = sqlx::MySqlPool::connect("mysql://root@localhost/chain_forensics")
        .await
        .unwrap();

    // Ensure DB migrations are applied
    sqlx::migrate!().run(&db_pool).await.unwrap();

    db_pool
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Setup DB
    let db_pool = setup_database().await;

    // Setup Bitcoin RPC
    // Default Bitcoin Core cookie path
    let mut cookie_filepath = r"~/.bitcoin/regtest/.cookie";
    if args.len() > 1 {
        cookie_filepath = &args[1];
    }

    let rpc_client = bitcoincore_rpc::Client::new(
        "http://localhost:18443",
        Auth::CookieFile(cookie_filepath.into()),
    )
    .expect("Could not connect to the Bitcoin RPC");

    // Setup Reqwest client
    let reqwest_client = reqwest::Client::new();

    // Initialize shared app state
    let app_state = Arc::new(AppState {
        rpc_client,
        reqwest_client,
        db_pool,
    });

    // Configure routing
    let app = Router::new()
        .route("/entities", get(entities))
        .route("/address/:address", get(address))
        .with_state(app_state);

    // Start HTTP server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
