extern crate dotenv;

use dotenv::dotenv;
use std::env;

/// A function used to extract an environment variable from the .env file
///
/// # Arguments
/// * 'key' - The environment variable to extract a value for
///
/// # Examples
/// ```
/// let redis_host = get_env_varaible("REDIS_HOST");
/// println!("REDIS_HOST = {}", redis_host);
/// ```
///
/// # Returns
/// A string containing the value of the environment variable or an empty string if the variable is not found
pub fn get_env_variable(key: &str) -> String {
    dotenv().ok();

    match env::var(key) {
        Ok(val) => val,
        Err(e) => {
            println!("{}: {}", key, e);
            String::from("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_env_variable_valid() {
        let test_cases = vec![("REDIS_HOST"), ("REDIS_PORT"), ("REDIS_PASSWORD")];

        for (key) in test_cases {
            let result = get_env_variable(key);
            assert!(!result.is_empty(), "Value for {} was empty", key);
        }
    }

    #[test]
    fn get_get_env_invalid_variable() {
        let test_cases = vec![("SOME_INVALID_VARIABLE_THAT_DOESNT_EXIST")];

        for (key) in test_cases {
            let result = get_env_variable(key);
            assert_eq!(result, "", "Value for {} was not expected", key);
        }
    }
}
