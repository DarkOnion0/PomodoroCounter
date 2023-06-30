use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Json, Router};
use pomolib::{Counter, Pomodoro};
mod types;
use crate::types::*;

#[tokio::main]
async fn main() {
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    let version = 1;
    Router::new()
        .route(
            &format!("/v{version}/pomodoro/:pomodoro/"),
            get(get_pomodoro),
        )
        .route(&format!("/v{version}/time/:time/"), get(get_time))
        .fallback(handler_404)
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

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use serde_json::{json, Value};
    use tower::ServiceExt;

    use super::*;

    /// Use custom values
    #[tokio::test]
    async fn get_pomodoro_1() {
        let app = app();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/pomodoro/2/?time=50&reset_point=4&short_pause=5&long_pause=20")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "Something bad happen while running the query"
        );
        let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            json!({
              "work_time": 100,
              "chill_time": 5,
              "spare_time": 0
            })
        );
    }
    #[tokio::test]
    /// Use the default values
    async fn get_pomodoro_2() {
        let app = app();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/pomodoro/2/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "Something bad happen while running the query"
        );
        let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            json!({
              "work_time": 50,
              "chill_time": 5,
              "spare_time": 0
            })
        );
    }

    /// Use custom values
    #[tokio::test]
    async fn get_time_1() {
        let app = app();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/time/7/?time=1&reset_point=4&short_pause=5&long_pause=20")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "Something bad happen while running the query"
        );
        let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            json!({
              "pomodoro": 2,
              "counter": {
                "work_time": 2,
                "chill_time": 5,
                "spare_time": 0
              }
            })
        );
    }
    /// Use the default values
    #[tokio::test]
    async fn get_time_2() {
        let app = app();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/time/54/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "Something bad happen while running the query"
        );
        let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            json!({
              "pomodoro": 1,
              "counter": {
                "work_time": 25,
                "chill_time": 0,
                "spare_time": 29
              }
            })
        );
    }
}
