use axum::extract::Path;
use axum::{routing::get, Json, Router};
use core::{Counter, Pomodoro};

#[tokio::main]
async fn main() {
    let version = 1;
    let app = Router::new().route(
        &format!("/v{version}/pomodoro/:pomodoro"),
        get(get_pomodoro),
    );

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
