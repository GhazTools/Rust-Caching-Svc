// EXTERNAL IMPORTS START HERE
use axum::response::Json;
use r2d2_redis::redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};
// EXTERNAL IMPORTS END HERE

// LOCAL IMPORTS START HERE
use crate::wrappers::redis_wrapper::REDIS_CLIENT;
// LOCAL IMPORTS END HERE

#[derive(Deserialize)]
pub struct GetVariableRequest {
    variable_name: String,
}

#[derive(Serialize)]
pub struct GetVariableResponse {
    variable_value: String,
}

pub async fn get_variable_request(request: Json<GetVariableRequest>) -> Json<GetVariableResponse> {
    let variable_name: &String = &request.variable_name;
    let mut connection = REDIS_CLIENT.get_connection().unwrap();

    let result: Result<String, RedisError> = connection.get(variable_name);

    match result {
        Ok(value) => Json(GetVariableResponse {
            variable_value: value,
        }),
        Err(e) => {
            eprintln!("Failed to set variable: {}", e);
            Json(GetVariableResponse {
                variable_value: "InnvalidResponseFromRedis".to_string(),
            })
        }
    }
}
