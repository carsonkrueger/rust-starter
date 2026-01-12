use crate::query_params::DatastarQueryParam;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(default)]
pub struct TableSearchParams {
    pub page: u32,
    pub limit: u32,
    pub query: Option<String>,
}

impl TableSearchParams {
    pub fn next_page(&self) -> Self {
        Self {
            page: self.page + 1,
            ..self.clone()
        }
    }
    pub fn reset(&self) -> Self {
        Self::default()
    }
}

impl Default for TableSearchParams {
    fn default() -> Self {
        Self {
            page: 1,
            limit: 20,
            query: None,
        }
    }
}

pub type DatastarSearchParams = DatastarQueryParam<TableSearchParams>;
