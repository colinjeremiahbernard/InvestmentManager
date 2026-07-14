use std::env;

#[derive(Clone)]
pub struct Settings {
    pub database_url: String,
    pub jwt_secret: String,
}

impl Settings {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),

            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        }
    }
}
