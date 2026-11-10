// Asynchronous REST API exposing the ticket management system.
// Endpoints:
//  - POST /tickets          -> create a ticket (from a draft)
//  - GET  /tickets/:id      -> retrieve ticket details
//  - PATCH /tickets/:id     -> patch title/description/status
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Deserialize)]
struct TicketDraft {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct TicketPatch {
    title: Option<String>,
    description: Option<String>,
    status: Option<Status>,
}

#[derive(Debug, Clone, Serialize)]
struct Ticket {
    id: u64,
    title: String,
    description: String,
    status: Status,
}

#[derive(Default)]
struct Store {
    tickets: HashMap<u64, Ticket>,
    next_id: u64,
}

type SharedStore = Arc<Mutex<Store>>;

fn validate_title(title: &str) -> Result<(), StatusCode> {
    if title.is_empty() || title.len() > 50 {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    Ok(())
}

fn validate_description(description: &str) -> Result<(), StatusCode> {
    if description.is_empty() || description.len() > 500 {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    Ok(())
}

async fn create_ticket(
    State(store): State<SharedStore>,
    Json(draft): Json<TicketDraft>,
) -> Result<(StatusCode, Json<Ticket>), StatusCode> {
    validate_title(&draft.title)?;
    validate_description(&draft.description)?;

    let mut store = store.lock().unwrap();
    let id = store.next_id;
    store.next_id += 1;
    let ticket = Ticket {
        id,
        title: draft.title,
        description: draft.description,
        status: Status::ToDo,
    };
    store.tickets.insert(id, ticket.clone());
    Ok((StatusCode::CREATED, Json(ticket)))
}

async fn get_ticket(
    State(store): State<SharedStore>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>, StatusCode> {
    store
        .lock()
        .unwrap()
        .tickets
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn patch_ticket(
    State(store): State<SharedStore>,
    Path(id): Path<u64>,
    Json(ticket_patch): Json<TicketPatch>,
) -> Result<Json<Ticket>, StatusCode> {
    if let Some(title) = &ticket_patch.title {
        validate_title(title)?;
    }
    if let Some(description) = &ticket_patch.description {
        validate_description(description)?;
    }

    let mut store = store.lock().unwrap();
    let ticket = store.tickets.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
    if let Some(title) = ticket_patch.title {
        ticket.title = title;
    }
    if let Some(description) = ticket_patch.description {
        ticket.description = description;
    }
    if let Some(status) = ticket_patch.status {
        ticket.status = status;
    }
    Ok(Json(ticket.clone()))
}

#[tokio::main]
async fn main() {
    let store: SharedStore = Arc::new(Mutex::new(Store::default()));
    let app = Router::new()
        .route("/tickets", post(create_ticket))
        .route("/tickets/:id", get(get_ticket))
        .route("/tickets/:id", patch(patch_ticket))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
