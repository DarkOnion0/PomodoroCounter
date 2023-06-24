use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Json, Router};
use core::{Counter, Pomodoro};
mod types;
use crate::types::*;

#[tokio::main]
async fn main() {
    let version = 1;
    let app = Router::new()
        .route(
            &format!("/v{version}/pomodoro/:pomodoro/"),
            get(get_pomodoro),
        )
        .route(&format!("/v{version}/time/:time/"), get(get_time))
        .fallback(handler_404);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Convert the requested number of pomodoro to time
async fn get_pomodoro(
    Path(pomodoro): Path<u32>,
    pagination: Option<Query<RequestParams>>,
) -> Json<Counter> {
    let Query(pagination) = pagination.unwrap_or_default();
    println!("{}", pagination.time);
    let mut args = Pomodoro {
        pomodoro,
        time: pagination.time,
        reset_point: pagination.reset_point,
        short_pause: pagination.short_pause,
        long_pause: pagination.long_pause,
    };
    Json(args.to_time())
}

async fn get_time(
    Path(time): Path<u32>,
    pagination: Option<Query<RequestParams>>,
) -> Json<PomodoroCounter> {
    let Query(pagination) = pagination.unwrap_or_default();
    let mut pomodoro = Pomodoro {
        pomodoro: 0,
        time: pagination.time,
        reset_point: pagination.reset_point,
        short_pause: pagination.short_pause,
        long_pause: pagination.long_pause,
    };
    let counter = pomodoro.to_pomodoro(time as i32);
    Json(PomodoroCounter {
        pomodoro: pomodoro.pomodoro,
        counter,
    })
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
