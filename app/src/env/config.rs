pub struct Config {
    pub port: i64,
}

impl Config {
    pub fn parse() -> Self {
        Self {
            port: dotenvy::var("PORT")
                .expect("Missing env: PORT")
                .parse()
                .expect("Invalid PORT"),
        }
    }
}
