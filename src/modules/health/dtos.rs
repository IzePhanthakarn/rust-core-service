use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct HealthData {
    pub version: String,
    pub environment: String,
    pub uptime_seconds: u64,
    pub timestamp: String,
    pub database: DbHealth,
}

#[derive(Serialize, ToSchema)]
pub struct DbHealth {
    pub status: String,
    pub latency_ms: u128,
    pub max_connections: u32,
    pub total_connections: u32,
    pub active_connections: u32,
    pub idle_connections: u32,
    pub db_size_mb: f64,
}