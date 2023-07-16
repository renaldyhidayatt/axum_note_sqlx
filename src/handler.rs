use crate::{
    abstract_trait::DynNoteService,
    errors::RequestError,
    request::{CreateNoteRequest, UpdateNoteRequest},
    response::NoteResponse,
};
use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};

type JsonResult<T> = Result<Json<T>, RequestError>;

pub async fn get_notes(
    Extension(note_service): Extension<DynNoteService>,
) -> JsonResult<Vec<NoteResponse>> {
    let notes = note_service.get_notes().await;

    match notes {
        Ok(note) => Ok(Json(note)),
        Err(err) => {
            tracing::error!("error retrieving tasks: {:?}", err);
            let error_response = RequestError::RunTimeError("Failed");
            Err(error_response)
        }
    }
}

pub async fn get_note_id(
    Extension(note_service): Extension<DynNoteService>,
    Path(id): Path<uuid::Uuid>,
) -> JsonResult<Option<NoteResponse>> {
    let note = note_service.get_note_id(id).await;

    match note {
        Ok(note) => Ok(Json(note)),
        Err(err) => {
            tracing::error!("error retrieving tasks: {:?}", err);
            let error_response = RequestError::RunTimeError("Failed");
            Err(error_response)
        }
    }
}

pub async fn create_note(
    Extension(note_service): Extension<DynNoteService>,
    payload: Json<CreateNoteRequest>,
) -> JsonResult<NoteResponse> {
    let created_note = note_service
        .create_note(&payload.title, &payload.content)
        .await;

    match created_note {
        Ok(note) => Ok(Json(note)),
        Err(err) => {
            tracing::error!("error retrieving tasks: {:?}", err);
            let error_response = RequestError::RunTimeError("Failed");
            Err(error_response)
        }
    }
}

pub async fn update_note(
    Extension(note_service): Extension<DynNoteService>,
    Path(id): Path<uuid::Uuid>,
    payload: Json<UpdateNoteRequest>,
) -> JsonResult<Option<NoteResponse>> {
    let updated_note = note_service
        .update_note(id, &payload.title, &payload.content)
        .await;

    match updated_note {
        Ok(note) => Ok(Json(note)),
        Err(err) => {
            tracing::error!("error retrieving tasks: {:?}", err);
            let error_response = RequestError::RunTimeError("Failed");
            Err(error_response)
        }
    }
}

pub async fn delete_note(
    Extension(note_service): Extension<DynNoteService>,
    Path(id): Path<uuid::Uuid>,
) -> JsonResult<()> {
    let delete_result = note_service.delete_note(id).await;

    match delete_result {
        Ok(()) => Ok(Json(())),
        Err(err) => {
            tracing::error!("error retrieving tasks: {:?}", err);
            let error_response = RequestError::RunTimeError("Failed");
            Err(error_response)
        }
    }
}

pub fn routes() -> Router {
    Router::new()
        .route("/notes", get(get_notes))
        .route("/notes/:id", get(get_note_id))
        .route("/notes", post(create_note))
        .route("/notes/:id", put(update_note))
        .route("/notes/:id", delete(delete_note))
}
