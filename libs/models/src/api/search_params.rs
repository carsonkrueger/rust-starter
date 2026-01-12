use datastar::templates::table::search_params::TableSearchParams;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
#[serde(default)]
pub struct SearchParams {
    pub page: u32,
    pub limit: u32,
    pub query: Option<String>,
}

impl SearchParams {
    pub fn sanitize(&self) -> Self {
        let mut params = self.clone();
        params.page = params.page.max(1);
        params.limit = params.limit.max(1).min(100);
        if let Some(q) = &mut params.query {
            q.truncate(500);
        }
        params
    }
    pub fn next_page(&self) -> Self {
        Self {
            page: self.page + 1,
            ..self.clone()
        }
    }
    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.limit) as i64
    }
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            page: 1,
            limit: 20,
            query: None,
        }
    }
}

impl Into<TableSearchParams> for SearchParams {
    fn into(self) -> TableSearchParams {
        TableSearchParams {
            page: self.page,
            limit: self.limit,
            query: self.query,
        }
    }
}

impl From<TableSearchParams> for SearchParams {
    fn from(params: TableSearchParams) -> Self {
        Self {
            page: params.page,
            limit: params.limit,
            query: params.query,
        }
        .sanitize()
    }
}
