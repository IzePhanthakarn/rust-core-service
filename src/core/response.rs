use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct EmptyData {}

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub status: String,
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedData<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    pub total_pages: i64,
    pub current_page: i64,
}

impl<T> PaginatedData<T> {
    pub fn new(items: Vec<T>, total_items: i64, page: i64, limit: i64) -> Self {
        Self { items, total_items, total_pages: total_pages(total_items, limit), current_page: page }
    }
}

pub fn normalize_page_limit(page: Option<i64>, limit: Option<i64>) -> (i64, i64) {
    let page = page.unwrap_or(1).max(1);
    let limit = limit.unwrap_or(10).clamp(1, 100);
    (page, limit)
}

pub fn total_pages(total_items: i64, limit: i64) -> i64 {
    (total_items as f64 / limit as f64).ceil() as i64
}

impl<T> ApiResponse<T> {
    pub fn success(code: u16, message: &str, data: T) -> Self {
        Self {
            status: "success".to_string(),
            code,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn success_without_data(code: u16, message: &str) -> Self {
        Self {
            status: "success".to_string(),
            code,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn fail(code: u16, message: &str) -> Self {
        Self {
            status: "fail".to_string(),
            code,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn error(code: u16, message: &str) -> Self {
        Self {
            status: "error".to_string(),
            code,
            message: message.to_string(),
            data: None,
        }
    }
}
