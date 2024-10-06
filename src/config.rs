#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: u64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_maxage: u64 = std::env::var("JWT_MAXAGE")
            .expect("JWT_MAXAGE must be set")
            .parse()
            .expect("JWT_MAXAGE must be a number");

        Config {
            database_url,
            jwt_secret,
            jwt_maxage: jwt_maxage,
            port: 8000,
        }
    }
}
