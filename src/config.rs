#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires: String,
    pub jwt_max_age: i32,
}

impl Config {
    pub fn new() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRES must be set");
        let jwt_max_age = std::env::var("JWT_MAXAGE").expect("JWT_MAX_AGE must set");
        Config {
            database_url,
            jwt_secret,
            jwt_expires,
            jwt_max_age: jwt_max_age.parse::<i32>().unwrap(),
        }
    }
}
