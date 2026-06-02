use diesel::prelude::*;
use diesel::sql_types::BigInt;
use std::time::Instant;

pub struct HealthRepository;

impl HealthRepository {
    // 1. ดึงขนาด Database (ฟังก์ชันเดิม)
    pub fn get_database_size(conn: &mut PgConnection) -> Result<i64, diesel::result::Error> {
        diesel::select(diesel::dsl::sql::<BigInt>("pg_database_size(current_database())"))
            .get_result::<i64>(conn)
    }

    // 2. ยิง Query เพื่อจับเวลา Ping (Latency)
    pub fn ping_database(conn: &mut PgConnection) -> u128 {
        let start = Instant::now();
        // ยิงคำสั่ง SELECT 1 ง่ายๆ เพื่อเทสต์การตอบสนอง
        let _ = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>("1"))
            .execute(conn);
        
        start.elapsed().as_millis()
    }
}