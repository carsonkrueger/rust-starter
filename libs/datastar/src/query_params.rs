use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, de};

pub struct DatastarQueryParam<T> {
    pub data: T,
}

impl<'de, T> Deserialize<'de> for DatastarQueryParam<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            datastar: String,
        }
        let raw = Raw::deserialize(deserializer)?;
        let data = serde_json::from_str(&raw.datastar).map_err(de::Error::custom)?;
        Ok(Self { data })
    }
}
