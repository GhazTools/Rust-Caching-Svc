// EXTERNAL IMPORTS START HERE
use lazy_static::lazy_static;
use r2d2_redis::{r2d2::Error, r2d2::Pool, r2d2::PooledConnection, RedisConnectionManager};
// EXTERNAL IMPORTS END HERE

// LOCAL IMPORTS START HERE
use super::dotenv_wrapper::get_env_variable;
// LOCAL IMPORTS END HERE

// We want to use the same client throughout the application, the pool manager will take care of the connections
lazy_static! {
    pub static ref REDIS_CLIENT: RedisClient = RedisClient::new();
}

pub struct RedisClient {
    pool: Pool<RedisConnectionManager>,
}

impl RedisClient {
    // INSTANTIATION LOGIC STARTS HERE
    pub fn new() -> Self {
        let redis_url = Self::get_connection_uri();
        let manager = RedisConnectionManager::new(redis_url).unwrap();
        let pool = Pool::builder().build(manager).unwrap();

        RedisClient { pool }
    }
    // INSTANTIATION LOGIC ENDS HERE

    // PUBLIC FUNCTIONS START HERE
    pub fn get_connection(&self) -> Result<PooledConnection<RedisConnectionManager>, Error> {
        self.pool.get()
    }
    // PUBLIC FUNCTIONS END HERE

    // PRIVATE FUNCTIONS START HERE
    fn get_connection_uri() -> String {
        let redis_host = get_env_variable("REDIS_HOST");
        let redis_port = get_env_variable("REDIS_PORT");
        let redis_password = get_env_variable("REDIS_PASSWORD");

        format!("redis://:{}@{}:{}", redis_password, redis_host, redis_port)
    }
    // PRIVATE FUNCTIONS END HERE
}
