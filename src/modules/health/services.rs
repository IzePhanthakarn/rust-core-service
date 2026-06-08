use chrono::Utc;
use diesel::PgConnection;
use std::time::Instant;

use crate::{
    config::database::DbPool,
    core::errors::AppError,
    modules::health::{
        dtos::{DbHealth, HealthData},
        repositories::HealthRepository,
    },
};

pub struct HealthService;

impl HealthService {
    pub fn get_system_health(
        conn: &mut PgConnection,
        pool: &DbPool,
        start_time: Instant,
    ) -> Result<HealthData, AppError> {
        // ข้อมูล Database
        let pool_state = pool.state();
        let total = pool_state.connections;
        let idle = pool_state.idle_connections;
        let active = total - idle;

        let db_size_bytes = HealthRepository::get_database_size(conn).unwrap_or(0);
        let db_size_mb = (db_size_bytes as f64) / (1024.0 * 1024.0);
        let latency_ms = HealthRepository::ping_database(conn);

        let db_health = DbHealth {
            status: "ok".to_string(),
            latency_ms,
            max_connections: pool.max_size(),
            total_connections: total,
            active_connections: active,
            idle_connections: idle,
            db_size_mb: (db_size_mb * 100.0).round() / 100.0,
        };

        let environment = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

        Ok(HealthData {
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment,
            uptime_seconds: start_time.elapsed().as_secs(),
            timestamp: Utc::now().to_rfc3339(),
            database: db_health,
        })
    }
}
