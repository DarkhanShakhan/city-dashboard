//! City Dashboard SSE Backend Server
//!
//! Provides:
//! - SSE endpoint at GET /events for real-time event streaming
//! - API endpoints for triggering events (POST /api/*)
//! - Automatic event broadcasting to all connected clients

mod events;

use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse, Response,
    },
    routing::{get, post},
    Json, Router,
};
use events::*;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};

/// Shared application state
#[derive(Clone)]
struct AppState {
    /// Broadcast channel for sending events to all SSE clients
    event_tx: broadcast::Sender<GameEvent>,
}

impl AppState {
    fn new() -> Self {
        // Create broadcast channel with capacity of 100 events
        let (tx, _) = broadcast::channel(100);
        Self { event_tx: tx }
    }

    /// Broadcast an event to all connected SSE clients
    fn broadcast(&self, event: GameEvent) {
        match self.event_tx.send(event.clone()) {
            Ok(receivers) => {
                info!("Event broadcast to {} clients: {:?}", receivers, event);
            }
            Err(_) => {
                warn!("No active SSE clients to receive event");
            }
        }
    }
}

// ============================================================================
// SSE Endpoint
// ============================================================================

/// SSE endpoint that streams events to clients
///
/// GET /events
async fn sse_handler(State(state): State<Arc<AppState>>) -> Sse<impl tokio_stream::Stream<Item = Result<Event, std::convert::Infallible>>> {
    info!("New SSE client connected");

    // Subscribe to broadcast channel
    let rx = state.event_tx.subscribe();
    let stream = BroadcastStream::new(rx);

    // Send initial connection event
    let initial_event = GameEvent::ConnectionStatus {
        connected: true,
        error: None,
    };
    let _ = state.event_tx.send(initial_event);

    // Convert broadcast stream to SSE event stream
    let event_stream = stream.filter_map(|result| match result {
        Ok(event) => {
            // Serialize event to JSON
            match serde_json::to_string(&event) {
                Ok(json) => Some(Ok(Event::default().data(json))),
                Err(e) => {
                    warn!("Failed to serialize event: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            warn!("Broadcast receive error: {}", e);
            None
        }
    });

    // Configure keep-alive to send heartbeat every 15 seconds
    // This prevents connection timeouts on idle connections
    Sse::new(event_stream).keep_alive(
        KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text("keepalive")
    )
}

// ============================================================================
// API Endpoints
// ============================================================================

/// POST /api/barrier/break
async fn barrier_break(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BarrierBrokenRequest>,
) -> Response {
    let event = GameEvent::BarrierBroken {
        team: req.team,
        message: req.message,
    };
    state.broadcast(event);
    (StatusCode::OK, "Event triggered").into_response()
}

/// POST /api/barrier/repair
async fn barrier_repair(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BarrierRepairedRequest>,
) -> Response {
    let event = GameEvent::BarrierRepaired { team: req.team };
    state.broadcast(event);
    (StatusCode::OK, "Event triggered").into_response()
}

/// POST /api/led/break
async fn led_break(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LedDisplayBrokenRequest>,
) -> Response {
    let event = GameEvent::LedDisplayBroken {
        team: req.team,
        message: req.message,
    };
    state.broadcast(event);
    (StatusCode::OK, "Event triggered").into_response()
}

/// POST /api/led/repair
async fn led_repair(State(state): State<Arc<AppState>>) -> Response {
    let event = GameEvent::LedDisplayRepaired;
    state.broadcast(event);
    (StatusCode::OK, "Event triggered").into_response()
}

/// POST /api/scada/compromise
async fn scada_compromise(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ScadaCompromisedRequest>,
) -> Response {
    let event = GameEvent::ScadaCompromised {
        building_id: req.building_id,
        team: req.team,
        message: req.message,
    };
    state.broadcast(event);
    (StatusCode::OK, "Event triggered").into_response()
}

/// POST /api/scada/restore
async fn scada_restore(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ScadaRestoredRequest>,
) -> Response {
    let event = GameEvent::ScadaRestored {
        building_id: req.building_id,
    };
    state.broadcast(event);
    (StatusCode::OK, "Event triggered").into_response()
}

/// POST /api/emergency/start
async fn emergency_start(
    State(state): State<Arc<AppState>>,
    Json(req): Json<EmergencyStopRequest>,
) -> Response {
    let event = GameEvent::EmergencyStop { reason: req.reason };
    state.broadcast(event);
    (StatusCode::OK, "Event triggered").into_response()
}

/// POST /api/emergency/stop
async fn emergency_stop(State(state): State<Arc<AppState>>) -> Response {
    let event = GameEvent::EmergencyStopDeactivated;
    state.broadcast(event);
    (StatusCode::OK, "Event triggered").into_response()
}

/// POST /api/danger/activate
async fn danger_activate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<DangerModeRequest>,
) -> Response {
    let event = GameEvent::DangerModeActivated { reason: req.reason };
    state.broadcast(event);
    (StatusCode::OK, "Event triggered").into_response()
}

/// POST /api/danger/deactivate
async fn danger_deactivate(State(state): State<Arc<AppState>>) -> Response {
    let event = GameEvent::DangerModeDeactivated;
    state.broadcast(event);
    (StatusCode::OK, "Event triggered").into_response()
}

/// POST /api/log
async fn log_message(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LogMessageRequest>,
) -> Response {
    let event = GameEvent::LogMessage {
        level: req.level,
        message: req.message,
    };
    state.broadcast(event);
    (StatusCode::OK, "Event triggered").into_response()
}

/// GET / - Info page
async fn index() -> Response {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>City Dashboard SSE Server</title>
    <style>
        body {
            font-family: monospace;
            background: #1e1e1e;
            color: #d4d4d4;
            padding: 20px;
            line-height: 1.6;
        }
        h1 { color: #4ec9b0; }
        h2 { color: #dcdcaa; }
        .endpoint { color: #ce9178; }
        .method { color: #569cd6; font-weight: bold; }
        code {
            background: #2d2d30;
            padding: 2px 6px;
            border-radius: 3px;
        }
        pre {
            background: #2d2d30;
            padding: 15px;
            border-left: 3px solid #007acc;
            overflow-x: auto;
        }
        ul { padding-left: 20px; }
        .example { margin: 10px 0; }
    </style>
</head>
<body>
    <h1>🏙️ City Dashboard SSE Server</h1>
    <p>Real-time event server for the City Dashboard application</p>

    <h2>SSE Endpoint</h2>
    <p><span class="method">GET</span> <span class="endpoint">/events</span></p>
    <p>Server-Sent Events stream. Connect from dashboard with:</p>
    <pre>SSE_URL=http://localhost:3000/events cargo run</pre>

    <h2>API Endpoints</h2>

    <h3>Barrier Events</h3>
    <div class="example">
        <p><span class="method">POST</span> <span class="endpoint">/api/barrier/break</span></p>
        <pre>curl -X POST http://localhost:3000/api/barrier/break \
  -H "Content-Type: application/json" \
  -d '{"team": "Red Team", "message": "Gate destroyed"}'</pre>
    </div>

    <div class="example">
        <p><span class="method">POST</span> <span class="endpoint">/api/barrier/repair</span></p>
        <pre>curl -X POST http://localhost:3000/api/barrier/repair \
  -H "Content-Type: application/json" \
  -d '{"team": "Blue Team"}'</pre>
    </div>

    <h3>LED Display Events</h3>
    <div class="example">
        <p><span class="method">POST</span> <span class="endpoint">/api/led/break</span></p>
        <pre>curl -X POST http://localhost:3000/api/led/break \
  -H "Content-Type: application/json" \
  -d '{"team": "Red Team", "message": "Display hacked"}'</pre>
    </div>

    <div class="example">
        <p><span class="method">POST</span> <span class="endpoint">/api/led/repair</span></p>
        <pre>curl -X POST http://localhost:3000/api/led/repair</pre>
    </div>

    <h3>SCADA Events</h3>
    <div class="example">
        <p><span class="method">POST</span> <span class="endpoint">/api/scada/compromise</span></p>
        <pre>curl -X POST http://localhost:3000/api/scada/compromise \
  -H "Content-Type: application/json" \
  -d '{"team": "Red Team", "building_id": 5, "message": "System hacked"}'</pre>
    </div>

    <div class="example">
        <p><span class="method">POST</span> <span class="endpoint">/api/scada/restore</span></p>
        <pre>curl -X POST http://localhost:3000/api/scada/restore \
  -H "Content-Type: application/json" \
  -d '{"building_id": null}'</pre>
    </div>

    <h3>Emergency Stop</h3>
    <div class="example">
        <p><span class="method">POST</span> <span class="endpoint">/api/emergency/start</span></p>
        <pre>curl -X POST http://localhost:3000/api/emergency/start \
  -H "Content-Type: application/json" \
  -d '{"reason": "Security breach"}'</pre>
    </div>

    <div class="example">
        <p><span class="method">POST</span> <span class="endpoint">/api/emergency/stop</span></p>
        <pre>curl -X POST http://localhost:3000/api/emergency/stop</pre>
    </div>

    <h3>Danger Mode</h3>
    <div class="example">
        <p><span class="method">POST</span> <span class="endpoint">/api/danger/activate</span></p>
        <pre>curl -X POST http://localhost:3000/api/danger/activate \
  -H "Content-Type: application/json" \
  -d '{"reason": "Hazardous materials detected"}'</pre>
    </div>

    <div class="example">
        <p><span class="method">POST</span> <span class="endpoint">/api/danger/deactivate</span></p>
        <pre>curl -X POST http://localhost:3000/api/danger/deactivate</pre>
    </div>

    <h3>Custom Log Message</h3>
    <div class="example">
        <p><span class="method">POST</span> <span class="endpoint">/api/log</span></p>
        <pre>curl -X POST http://localhost:3000/api/log \
  -H "Content-Type: application/json" \
  -d '{"level": "critical", "message": "Custom event message"}'</pre>
    </div>

    <h2>Testing</h2>
    <p>Watch SSE stream:</p>
    <pre>curl -N http://localhost:3000/events</pre>
</body>
</html>"#;

    (StatusCode::OK, [(header::CONTENT_TYPE, "text/html")], html).into_response()
}

// ============================================================================
// Main Application
// ============================================================================

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create shared state
    let state = Arc::new(AppState::new());

    // Configure CORS to allow requests from anywhere
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/", get(index))
        .route("/events", get(sse_handler))
        // Barrier endpoints
        .route("/api/barrier/break", post(barrier_break))
        .route("/api/barrier/repair", post(barrier_repair))
        // LED display endpoints
        .route("/api/led/break", post(led_break))
        .route("/api/led/repair", post(led_repair))
        // SCADA endpoints
        .route("/api/scada/compromise", post(scada_compromise))
        .route("/api/scada/restore", post(scada_restore))
        // Emergency endpoints
        .route("/api/emergency/start", post(emergency_start))
        .route("/api/emergency/stop", post(emergency_stop))
        // Danger mode endpoints
        .route("/api/danger/activate", post(danger_activate))
        .route("/api/danger/deactivate", post(danger_deactivate))
        // Log endpoint
        .route("/api/log", post(log_message))
        .layer(cors)
        .with_state(state);

    // Start server
    let addr = "0.0.0.0:3000";
    info!("🚀 Server starting on http://{}", addr);
    info!("📡 SSE endpoint: http://{}/events", addr);
    info!("📝 API docs: http://{}/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
