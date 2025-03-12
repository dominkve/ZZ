use axum::{
    extract::{State, Json, Path},
    routing::{ post, get },
    Router,
};
use std::sync::Arc;
use sqlx::{
    postgres::PgPoolOptions,
    PgPool,
};
use serde::Serialize;
use serde_json::{Value, json};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use urlencoding::decode;
use uuid::Uuid;
use bytes::Bytes;
//use futures::StreamExt;
use async_nats::jetstream;

#[derive(sqlx::FromRow, Debug, Serialize)]
struct LanguageList {
    language: String,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
struct ProblemList {
    id: i16,
    name: String,
}
#[derive(sqlx::FromRow, Debug, Serialize)]
struct Problem {
    id: i16,
    name: String,
    description: String,
}

struct AppState {
    db: PgPool,
    nats_client: Arc<jetstream::Context>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the NATS server
    let nats_client = async_nats::connect("nats://172.30.221.102:4222").await?;
    let jetstream = async_nats::jetstream::new(nats_client);

    // Create a connection pool
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://domin:Dominik100@localhost:5432/domin")
        .await?;

    // Wrap the pool in an Arc to share across requests
    let shared_state = Arc::new(AppState {
        db,
        nats_client: Arc::new(jetstream),
    });

    // Define CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allows all origins
        .allow_methods(Any)
        .allow_headers(Any);

    // router
    let app = Router::new()
        .route("/languages", get(get_languages))
        .route("/execute", post(execute))
        .route("/problems", get(get_problems))
        .route("/problems/{name}", get(get_problem))
        .with_state(shared_state)
        .layer(cors); // Apply CORS middleware, it blocks me otherwise, they are on different ports?

    // listener
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server running on https://{}", addr);

    // Start server
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn execute(
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<Value>
    ) -> Json<Value> {
    dbg!(&payload);

    let id = Uuid::new_v4();


    if let Value::Object(ref mut map) = payload {
        map.insert("job_id".to_string(), json!(id.to_string()));
    }
    dbg!(&payload);

    // Serialize JSON to a byte vector
    let json_bytes = serde_json::to_vec(&payload).expect("Failed to serialize JSON");

    // Convert the byte vector into a 'Bytes' object
    let binary_data = Bytes::from(json_bytes);

    state.nats_client.publish("requests.execution", binary_data.clone()).await.unwrap();
    Json(json!({ "job_id": id }))
}

async fn get_languages(State(state): State<Arc<AppState>>) -> Json<Vec<LanguageList>> {
    let languages: Vec<LanguageList> = sqlx::query_as::<_, LanguageList>("SELECT language FROM languages")
        .fetch_all(&state.db)
        .await
        .expect("Failed query.");
    
    for language in &languages {
        dbg!(language);
    }
    Json(languages)
}


async fn get_problems(State(state): State<Arc<AppState>>) -> Json<Vec<ProblemList>> {
    let problems: Vec<ProblemList> = sqlx::query_as::<_, ProblemList>("SELECT id, name FROM problems")
        .fetch_all(&state.db)
        .await
        .expect("Failed query");

    for problem in &problems {
        println!("{:#?}", problem);
    }

    Json(problems)
}

async fn get_problem(Path(name): Path<String>, State(state): State<Arc<AppState>>) -> Json<Problem> {
    dbg!("getting problem{}", &name);
    let name = decode(&name).unwrap();
    let result = sqlx::query_as::<_, Problem>("SELECT id, name, description FROM problems WHERE name = $1")
        .bind(&name)
        .fetch_one(&state.db)
        .await
        .expect("Failed query");

    dbg!(&result);
    Json(result)
}