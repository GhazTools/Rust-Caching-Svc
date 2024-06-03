// EXTERNAL IMPORTS START HERE
use axum::response::Json;
use serde::Serialize;

// EXTERNAL IMPORTS END HERE

// LOCAL IMPORTS START HERE
// LOCAL IMPORTS END HERE

#[derive(Serialize)]
pub struct ServiceStatusResponse {
    service_status: bool,
}

pub async fn service_status_request() -> Json<ServiceStatusResponse> {
    Json(ServiceStatusResponse {
        service_status: true,
    })
}
