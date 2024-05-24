use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::{response::Json, Extension};
use common::db::DB;
use hyper::{header, HeaderMap};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize, JsonSchema)]
pub struct Health {
    status: HealthStatus,
}

#[derive(Eq, Debug, Hash, PartialEq, Serialize, JsonSchema)]
pub enum HealthStatus {
    Healthy,
    Degraded,
}

pub async fn run_healthcheck(Extension(db): Extension<DB>) -> impl IntoApiResponse {
    let mut health = Health {
        status: HealthStatus::Healthy,
    };
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CACHE_CONTROL,
        "no-store, no-cache, max-age=0, must-revalidate, proxy-revalidate"
            .parse()
            .unwrap(),
    );
    let db_check = sqlx::query("SELECT 1").fetch_one(&db).await;

    if let Err(db_health) = db_check {
        tracing::error!("{:?}", db_health);
        health.status = HealthStatus::Degraded;
    }
    (headers, Json(health))
}

pub fn run_healthcheck_docs(op: TransformOperation) -> TransformOperation {
    op.description("Health status")
}
