use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct DatastarParams<T> {
    pub datastar: T,
}
