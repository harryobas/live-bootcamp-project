use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;



lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
    pub static ref DATABASE_URL: String = set_db_url();
}

fn set_token() -> String {
    dotenv().ok();
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR)
        .expect("JWT secrete must be set");

    if secret.is_empty() {
        panic!("JWT secret must not be empty")
    }

    secret
}

fn set_db_url() -> String {
    dotenv().ok();
    let db_url = std_env::var(env::DATABASE_URL_ENV_VAR)
        .expect("DB url must be set");
    if db_url.is_empty() {
        panic!("DB url must not be empty")
    }

    db_url

}

mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";

}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}


pub const JWT_COOKIE_NAME: &str = "jwt";