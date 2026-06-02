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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::get, Router};
    use axum::http::{Request, StatusCode};
    use axum::body::Body;
    use tower::ServiceExt; // นำเข้าเครื่องมือสำหรับจำลองการยิง API
    
    // นำเข้าของที่ต้องใช้สร้าง State
    use crate::AppState; 
    use crate::config;
    use std::time::Instant;

    #[tokio::test]
    async fn test_health_check_api() {
        // 0. โหลดตัวแปรจาก .env ก่อน เพื่อให้สร้าง DB Pool ได้โดยไม่พัง
        dotenvy::dotenv().ok();

        // 1. สร้าง State แบบเดียวกับใน main.rs
        let db_pool = config::database::establish_connection_pool();
        let state = AppState {
            db_pool,
            start_time: Instant::now(),
        };

        // 2. สร้าง Router จำลอง และ **ยัด State เข้าไป** (ตรงนี้คือจุดที่แก้ Error ครับ!)
        let app = Router::new()
            .route("/health", get(health_check))
            .with_state(state); // << เปลี่ยน Router<AppState> เป็น Router<()> ให้สมบูรณ์

        // 3. สร้าง Request ปลอมขึ้นมา (GET /health)
        let request = Request::builder()
            .method("GET")
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        // 4. ยิง Request ปลอมเข้าไป
        let response = app.oneshot(request).await.unwrap();

        // 5. ตรวจสอบผลลัพธ์ว่าต้องได้ Status 200 OK
        assert_eq!(response.status(), StatusCode::OK);
    }
}