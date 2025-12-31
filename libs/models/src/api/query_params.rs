use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(default)]
pub struct QueryParams {
    pub page: u32,
    pub limit: u32,
    pub query: Option<String>,
}

impl QueryParams {
    pub fn sanitize(&self) -> Self {
        let mut params = self.clone();
        params.page = params.page.max(1);
        params.limit = params.limit.max(1).min(100);
        if let Some(q) = &mut params.query
            && q.len() > 500
        {
            q.truncate(500);
        }
        params
    }
}

impl Default for QueryParams {
    fn default() -> Self {
        Self {
            page: 1,
            limit: 10,
            query: None,
        }
    }
}
