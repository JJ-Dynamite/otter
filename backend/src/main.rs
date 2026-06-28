use axum::{
    routing::{get, post},
    Router, Json, response::IntoResponse,
    extract::Multipart,
};
use tower_http::cors::{CorsLayer, Any};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
struct HealthResponse { status: String, service: String, version: String }
#[derive(Serialize)]
struct RootResponse { service: String, version: String, description: String, endpoints: Vec<String> }
#[derive(Serialize)]
struct TranscriptResponse { id: String, status: String, title: String, duration: String, segments: Vec<TranscriptSegment> }
#[derive(Serialize)]
struct TranscriptSegment { start: f64, end: f64, speaker: String, text: String }

async fn health() -> impl IntoResponse {
    Json(HealthResponse { status: "healthy".into(), service: "otter".into(), version: "0.1.0".into() })
}

async fn root() -> impl IntoResponse {
    Json(RootResponse {
        service: "otter".into(), version: "0.1.0".into(),
        description: "Auto-transcribe meetings".into(),
        endpoints: vec!["GET /health".into(), "POST /transcribe".into(), "GET /transcripts/:id".into()],
    })
}

async fn transcribe_meeting(mut multipart: Multipart) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let _name = field.file_name().unwrap_or("unknown").to_string();
        let _data = field.bytes().await.unwrap();
    }
    let segments = vec![
        TranscriptSegment { start: 0.0, end: 3.0, speaker: "Alice".into(), text: "Let’s start with the Q1 review.".into() },
        TranscriptSegment { start: 3.0, end: 6.5, speaker: "Bob".into(), text: "Sure, revenue is up 15% over last quarter.".into() },
        TranscriptSegment { start: 6.5, end: 10.0, speaker: "Alice".into(), text: "Great, let’s discuss the roadmap next.".into() },
    ];
    Json(TranscriptResponse { id, status: "completed".into(), title: "Meeting Transcript".into(), duration: "10:00".into(), segments })
}

async fn get_transcript(axum::extract::Path(id): axum::extract::Path<String>) -> impl IntoResponse {
    Json(TranscriptResponse {
        id, status: "completed".into(), title: "Meeting Transcript".into(), duration: "10:00".into(),
        segments: vec![TranscriptSegment { start: 0.0, end: 3.0, speaker: "Alice".into(), text: "Let’s start with the Q1 review.".into() }],
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let app = Router::new()
        .route("/", get(root)).route("/health", get(health))
        .route("/transcribe", post(transcribe_meeting)).route("/transcripts/:id", get(get_transcript))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("otter backend running on port 3001");
    axum::serve(listener, app).await.unwrap();
}
