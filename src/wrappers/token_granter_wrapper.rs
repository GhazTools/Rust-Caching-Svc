// EXTERNAL IMPORTS START HERE
use lazy_static::lazy_static;
use serde::Deserialize;
use tracing::info;

// EXTERNAL IMPORTS END HERE

// STANDARD IMPORTS START HERE
use std::collections::HashMap;
// STANDARD IMPORTS END HERE

// LOCAL IMPORTS START HERE
use super::dotenv_wrapper::get_env_variable;
// LOCAL IMPORTS END HERE

lazy_static! {
    pub static ref TOKEN_GRANTER_CLIENT: TokenGranterWrapper = TokenGranterWrapper::new();
}

pub struct TokenGranterWrapper {
    token_granter_url: String,
}

#[allow(non_snake_case)] // This is necessary because the response from the token granter is in PascalCase
#[derive(Deserialize)]
struct TokenGranterResponse {
    ErrorCode: i8,
}

impl TokenGranterWrapper {
    // INSTANTIATION LOGIC STARTS HERE
    fn new() -> Self {
        let token_granter_url = get_env_variable("TOKEN_GRANTER_URL");

        TokenGranterWrapper { token_granter_url }
    }
    // INSTANTIATION LOGIC ENDS HERE

    // PUBLIC FUNCTIONS START HERE
    pub async fn validate_token(&self, username: &str, token: &str) -> bool {
        let url = format!("{}/token/validate", self.token_granter_url);
        let client = reqwest::Client::new();

        let map = HashMap::from([("username", username), ("token", token)]);
        let response_result = client.post(&url).json(&map).send().await;

        if let Ok(response) = response_result {
            if let Ok(response_text) = response.text().await {
                let token_granter_response: TokenGranterResponse =
                    serde_json::from_str(&response_text).unwrap();
                Self::check_response(token_granter_response)
            } else {
                false
            }
        } else {
            false
        }
    }
    // PUBLIC FUNCTIONS END HERE

    // PRIVATE FUNCTIONS START HERE
    fn check_response(response: TokenGranterResponse) -> bool {
        if response.ErrorCode == 0 {
            true
        } else {
            info!(
                "Token validation failed with error code: {}",
                response.ErrorCode
            );
            false
        }
    }
    // PRIVATE FUNCTIONS END HERE
}
