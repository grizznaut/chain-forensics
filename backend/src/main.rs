use axum::{extract::State, routing::get, Json, Router};
use bitcoin::TxOut;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;

struct AppState {
    rpc_client: Client,
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

// Index chain data.
// TODO: This should index inflows/outflows of funds per address, and group addresses
// into clusters ("entities") based on common-input spend, writing results to database.
fn index_blockchain(state: Arc<AppState>) {
    let start_height = 0;
    let end_height = state.rpc_client.get_block_count().unwrap();

    let mut utxos_by_script_pubkey: HashMap<String, Vec<TxOut>> = HashMap::new();
    for block_height in start_height..end_height {
        let blockhash = state.rpc_client.get_block_hash(block_height).unwrap();
        let block = state.rpc_client.get_block(&blockhash).unwrap();

        for tx in block.txdata {
            for txout in tx.output {
                utxos_by_script_pubkey
                    .entry(txout.script_pubkey.to_hex_string())
                    .and_modify(|e| e.push(txout.clone()))
                    .or_insert(vec![txout]);
                // TODO: write data to DB
            }
        }
    }

    dbg!(utxos_by_script_pubkey.len());
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

    let rpc_client = Client::new(
        "http://localhost:18443",
        Auth::CookieFile(cookie_filepath.into()),
    )
    .expect("Could not connect to the Bitcoin RPC");

    // Initialize shared app state
    let app_state = Arc::new(AppState {
        rpc_client,
        db_pool,
    });

    // Run indexer
    // TODO: this should only run the first time the app is started.
    index_blockchain(Arc::clone(&app_state));

    // Configure routing
    let app = Router::new()
        .route("/entities", get(entities))
        .with_state(app_state);

    // Start HTTP server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
