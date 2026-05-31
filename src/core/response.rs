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

    pub fn error(code: u16, message: &str) -> Self {
        Self {
            status: "error".to_string(),
            code,
            message: message.to_string(),
            data: None, // Error ไม่ต้องมี Data ส่งกลับไป
        }
    }
}