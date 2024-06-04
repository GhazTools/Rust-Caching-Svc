// EXTERNAL IMPORTS START HERE
use axum::response::Json;
use r2d2_redis::redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
// EXTERNAL IMPORTS END HERE

// LOCAL IMPORTS START HERE
use crate::wrappers::redis_wrapper::REDIS_CLIENT;
use crate::wrappers::token_granter_wrapper::TOKEN_GRANTER_CLIENT;
// LOCAL IMPORTS END HERE

#[derive(Deserialize)]
pub struct GetVariableRequest {
    username: String,
    token: String,
    variable_name: String,
}

#[derive(Serialize)]
pub struct GetVariableResponse {
    variable_value: String,
}

pub async fn get_variable_request(request: Json<GetVariableRequest>) -> Json<GetVariableResponse> {
    if !TOKEN_GRANTER_CLIENT
        .validate_token(&request.username, &request.token)
        .await
    {
        return Json(GetVariableResponse {
            variable_value: "InvalidToken".to_string(),
        });
    }

    let variable_name: &String = &request.variable_name;
    let mut connection = REDIS_CLIENT.get_connection().unwrap();

    let result: Result<String, RedisError> = connection.get(variable_name);

    match result {
        Ok(value) => {
            info!("Successfully retrieved variable: {}", variable_name);
            Json(GetVariableResponse {
                variable_value: value,
            })
        }
        Err(e) => {
            error!(
                "Failed to retrieve variable: {}. Error: {}",
                variable_name, e
            );
            Json(GetVariableResponse {
                variable_value: "InnvalidResponseFromRedis".to_string(),
            })
        }
    }
}
