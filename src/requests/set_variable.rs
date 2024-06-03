// EXTERNAL IMPORTS START HERE
use axum::response::Json;
use r2d2_redis::redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
// EXTERNAL IMPORTS END HERE

// LOCAL IMPORTS START HERE
use crate::wrappers::redis_wrapper::REDIS_CLIENT;
// LOCAL IMPORTS END HERE

#[derive(Deserialize)]
pub struct SetVariableRequest {
    variable_name: String,
    variable_value: String,
}

#[derive(Serialize)]
pub struct SetVariableResponse {
    successful: bool,
}

pub async fn set_variable_request(request: Json<SetVariableRequest>) -> Json<SetVariableResponse> {
    let mut connection = REDIS_CLIENT.get_connection().unwrap();
    let variable_name: &String = &request.variable_name;
    let variable_value: &String = &request.variable_value;

    let result: Result<(), RedisError> = connection.set(variable_name, variable_value);

    match result {
        Ok(_) => {
            info!("Successfully set variable: {}", variable_name);
            Json(SetVariableResponse { successful: true })
        }
        Err(e) => {
            error!("Failed to set variable: {}. Error: {}", variable_name, e);
            Json(SetVariableResponse { successful: false })
        }
    }
}
