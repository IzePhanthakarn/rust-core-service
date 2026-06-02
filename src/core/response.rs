use serde::Serialize;
use utoipa::ToSchema;

// สร้างตัวแทนของ Data ว่างๆ เพื่อให้ Utoipa นำไปเจน Docs ได้
#[derive(Serialize, ToSchema)]
pub struct EmptyData {}

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub status: String,
    pub code: u16,
    pub message: String,
    // ถ้าไม่มี data จะไม่แสดงฟิลด์นี้ (เพื่อความสะอาดของ JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

// เพิ่มตัวนี้สำหรับรับ Query String: ?page=1&limit=10
// #[derive(Deserialize, utoipa::IntoParams)]
// pub struct PaginationQuery {
//     pub page: Option<i64>,
//     pub limit: Option<i64>,
// }

// เพิ่มตัวนี้สำหรับห่อข้อมูลแบบแบ่งหน้า
#[derive(Serialize, ToSchema)]
pub struct PaginatedData<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    pub total_pages: i64,
    pub current_page: i64,
}

impl<T> ApiResponse<T> {
    // Helper function สำหรับสร้าง Success Response ง่ายๆ
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
            data: None, // พอเป็น None ฟิลด์ data จะหายไปจาก JSON เลย
        }
    }

    // สำหรับปัญหาที่เกิดจากฝั่ง Client (HTTP 4xx)
    pub fn fail(code: u16, message: &str) -> Self {
        Self {
            status: "fail".to_string(),
            code,
            message: message.to_string(),
            data: None,
        }
    }

    // สำหรับปัญหาที่เกิดจากระบบเราเอง (HTTP 5xx)
    pub fn error(code: u16, message: &str) -> Self {
        Self {
            status: "error".to_string(),
            code,
            message: message.to_string(),
            data: None,
        }
    }
}
