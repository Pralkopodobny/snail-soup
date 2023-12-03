use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: u32,
}

impl Config {
    pub fn init() -> Config {
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        Config {
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: match jwt_maxage.parse::<u32>() {
                Err(_) => panic!("JWT_MAXAGE must be an integer value"),
                Ok(val) => val
            }
        }
    }
}