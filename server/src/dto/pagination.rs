use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}
