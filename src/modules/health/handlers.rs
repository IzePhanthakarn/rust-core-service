use axum::{extract::State, Json};
use serde::Serialize;
use chrono::Utc;
use utoipa::ToSchema;
use crate::{AppState, core::response::ApiResponse};

// เปลี่ยนชื่อจาก HealthResponse เป็น HealthData เพราะมันคือเนื้อหาข้างใน
#[derive(Serialize, ToSchema)]
pub struct HealthData {
    pub version: String,
    pub uptime_seconds: u64,
    pub timestamp: String,
    pub database: DbHealth,
}

#[derive(Serialize, ToSchema)]
pub struct DbHealth {
    pub status: String,
    pub max_connections: u32,
    pub total_connections: u32,
    pub active_connections: u32,
    pub idle_connections: u32,
}

// === เพิ่มส่วนนี้เพื่อสร้าง API Docs ===
#[utoipa::path(
    get,
    path = "/health",
    tag = "System Health",
    responses(
        (status = 200, description = "System is healthy", body = ApiResponse<HealthData>)
    )
)]
// ===================================
pub async fn health_check(State(state): State<AppState>) -> Json<ApiResponse<HealthData>> {
    let pool_state = state.db_pool.state();
    
    let total = pool_state.connections;
    let idle = pool_state.idle_connections;
    let active = total - idle;
    
    let db_health = DbHealth {
        status: "ok".to_string(),
        max_connections: 15,
        total_connections: total,
        active_connections: active,
        idle_connections: idle,
    };

    let data = HealthData {
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: state.start_time.elapsed().as_secs(),
        timestamp: Utc::now().to_rfc3339(),
        database: db_health,
    };

    // ใช้ Helper function ส่งข้อมูลกลับไป
    Json(ApiResponse::success(200, "Server is healthy", data))
}