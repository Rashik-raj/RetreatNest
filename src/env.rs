use const_str;

use dotenvy_macro::dotenv;

pub struct Env {
    pub server_host: &'static str,
    pub server_port: &'static str,
    pub database_url: &'static str,
    pub password_salt: &'static str,
    pub jwt_access_key: &'static str,
    pub jwt_access_lifetime_in_min: u64,
    pub jwt_refresh_key: &'static str,
    pub jwt_refresh_lifetime_in_min: u64,
}

pub const ENV: Env = Env {
    server_host: dotenv!("SERVER_HOST"),
    server_port: dotenv!("SERVER_PORT"),
    database_url: dotenv!("DATABASE_URL"),
    password_salt: dotenv!("PASSWORD_SALT"),
    jwt_access_key: dotenv!("JWT_ACCESS_KEY"),
    jwt_access_lifetime_in_min: const_str::parse!(dotenv!("JWT_ACCESS_LIFETIME_IN_MIN"), u64),
    jwt_refresh_key: dotenv!("JWT_REFRESH_KEY"),
    jwt_refresh_lifetime_in_min: const_str::parse!(dotenv!("JWT_REFRESH_LIFETIME_IN_MIN"), u64),
};

