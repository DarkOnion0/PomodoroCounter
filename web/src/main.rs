use axum::extract::Path;
use axum::{routing::get, Json, Router};
use core::{Counter, Pomodoro};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let version = 1;
    let app = Router::new()
        .route(
            &format!("/v{version}/pomodoro/:pomodoro"),
            get(get_pomodoro),
        )
        .route(&format!("/v{version}/time/:time"), get(get_time));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Convert the requested number of pomodoro to time
async fn get_pomodoro(Path(pomodoro): Path<u32>) -> Json<Counter> {
    let mut args = Pomodoro::new(pomodoro);
    Json(args.to_time())
}

async fn get_time(Path(time): Path<u32>) -> Json<PomodoroCounter> {
    let mut pomodoro = Pomodoro::new(0);
    let counter = pomodoro.to_pomodoro(time as i32);
    Json(PomodoroCounter {
        pomdoro: pomodoro.pomodoro,
        counter: counter,
    })
}

#[derive(Serialize)]
struct PomodoroCounter {
    pomdoro: u32,
    counter: Counter,
}
