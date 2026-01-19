#[derive(Clone, Debug)]
pub struct DBConfig {
    pub port: i64,
    pub user: String,
    password: String,
    pub db_name: String,
    pub max_conns: u32,
    pub host: String,
}

impl DBConfig {
    pub fn parse(internal: bool) -> Self {
        Self {
            host: dotenvy::var("DB_HOST")
                .expect("Missing env: DB_HOST")
                .parse()
                .expect("Invalid DB_HOST"),
            port: match internal {
                false => dotenvy::var("DB_PORT")
                    .expect("Missing env: DB_PORT")
                    .parse()
                    .expect("Invalid DB_PORT"),
                true => 5432,
            },
            db_name: dotenvy::var("DB_NAME")
                .expect("Missing env: DB_NAME")
                .parse()
                .expect("Invalid DB_NAME"),
            user: dotenvy::var("DB_USER")
                .expect("Missing env: DB_USER")
                .parse()
                .expect("Invalid DB_USER"),
            password: dotenvy::var("DB_PASSWORD")
                .expect("Missing env: DB_PASSWORD")
                .parse()
                .expect("Invalid DB_PASSWORD"),
            max_conns: dotenvy::var("DB_MAX_CONNS")
                .unwrap_or("10".to_string())
                .parse()
                .expect("Invalid DB_MAX_CONNS"),
        }
    }
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        )
    }
}
