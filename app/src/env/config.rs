use std::path::Path;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: i64,
}

impl Config {
    pub fn parse<P: AsRef<Path>>(env_path: Option<P>) -> Self {
        if let Some(ap) = env_path {
            if let Some(p) = ap.as_ref().to_str() {
                dotenvy::from_path(p).expect(&format!("Could not parse env: {:?}", p))
            }
        }
        Self {
            port: dotenvy::var("PORT")
                .expect("Missing env: PORT")
                .parse()
                .expect("Invalid PORT"),
        }
    }
}
