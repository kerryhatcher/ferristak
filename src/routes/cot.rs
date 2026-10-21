//! Cursor-on-Target event endpoints.
//!
//! Events can be submitted and retrieved as JSON or as canonical CoT XML,
//! selected by `Content-Type` on write and `?format=xml` / `Accept` on read.

use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{auth::AuthUser, error::AppError, models::cot, state::AppState};

/// Query string for `GET /api/v1/cot/events`.
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    /// Maximum number of events (1-500).
    pub limit: Option<i64>,
    /// Only return events with `send_time` at or after this RFC 3339 timestamp.
    pub since: Option<DateTime<Utc>>,
    /// Only return events for this CoT uid.
    pub uid: Option<String>,
    /// `xml` to receive CoT XML instead of JSON.
    pub format: Option<String>,
}

/// Query string for endpoints that only need a format override.
#[derive(Debug, Deserialize)]
pub struct FormatQuery {
    pub format: Option<String>,
}

/// `POST /api/v1/cot/events` — submit a CoT event.
pub async fn create(
    State(state): State<AppState>,
    caller: AuthUser,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    let xml_request = content_type.contains("xml");

    let input = if xml_request {
        let text = std::str::from_utf8(&body)
            .map_err(|_| AppError::validation("request body must be UTF-8"))?;
        cot::from_xml(text)?
    } else {
        serde_json::from_slice::<cot::NewCotEvent>(&body)
            .map_err(|err| AppError::validation(format!("invalid CoT JSON: {err}")))?
    };

    input.validate()?;

    let uid = input.uid_or_default(caller.id);
    let event = cot::insert(&state.db, caller.id, &uid, &input).await?;

    let mut response = render_event(&event, xml_request)?;
    *response.status_mut() = StatusCode::CREATED;
    Ok(response)
}

/// `GET /api/v1/cot/events` — the event feed, newest first.
pub async fn list(
    State(state): State<AppState>,
    _caller: AuthUser,
    headers: HeaderMap,
    Query(query): Query<ListQuery>,
) -> Result<Response, AppError> {
    let limit = query.limit.unwrap_or(100).clamp(1, 500);
    let events = cot::list(&state.db, limit, query.since, query.uid.as_deref()).await?;

    render_events(&events, wants_xml(query.format.as_deref(), &headers))
}

/// `GET /api/v1/cot/events/{id}` — a single event.
pub async fn get(
    State(state): State<AppState>,
    _caller: AuthUser,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Query(query): Query<FormatQuery>,
) -> Result<Response, AppError> {
    let event = cot::find(&state.db, id).await?.ok_or(AppError::NotFound)?;
    render_event(&event, wants_xml(query.format.as_deref(), &headers))
}

/// `GET /api/v1/cot/events/latest` — newest event per uid (the current picture).
pub async fn latest(
    State(state): State<AppState>,
    _caller: AuthUser,
    headers: HeaderMap,
    Query(query): Query<FormatQuery>,
) -> Result<Response, AppError> {
    let events = cot::latest(&state.db, 500).await?;
    render_events(&events, wants_xml(query.format.as_deref(), &headers))
}

fn render_event(event: &cot::CotEvent, xml: bool) -> Result<Response, AppError> {
    if xml {
        let body = cot::to_xml(event)?;
        Ok((
            [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
            body,
        )
            .into_response())
    } else {
        Ok(Json(event).into_response())
    }
}

fn render_events(events: &[cot::CotEvent], xml: bool) -> Result<Response, AppError> {
    if xml {
        let body = cot::to_xml_list(events)?;
        Ok((
            [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
            body,
        )
            .into_response())
    } else {
        Ok(Json(events).into_response())
    }
}

fn wants_xml(format: Option<&str>, headers: &HeaderMap) -> bool {
    if format.is_some_and(|format| format.eq_ignore_ascii_case("xml")) {
        return true;
    }
    headers
        .get(header::ACCEPT)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|accept| accept.contains("xml"))
}
