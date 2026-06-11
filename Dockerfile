# ==========================================
# Stage 1: Builder (ใช้ Image ตัวใหญ่เพื่อ Compile โค้ด)
# ==========================================
FROM rust:1-slim-bookworm AS builder

WORKDIR /usr/src/app

# ติดตั้ง libpq-dev เพราะแอปต้องเชื่อมต่อ PostgreSQL (Diesel/SQLx ต้องการใช้ตอน Build)
RUN apt-get update \
    && apt-get install -y --no-install-recommends libpq-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy ไฟล์ทั้งหมดเข้าไป
COPY . .

# สั่ง Build โค้ดแบบ Release (จะใช้เวลานานหน่อยในขั้นตอนนี้)
# สมมติว่าโปรเจกต์คุณชื่อ rust-core-service ในไฟล์ Cargo.toml
RUN cargo build --release

# ==========================================
# Stage 2: Runner (ใช้ Image ตัวเล็กมากเพื่อนำไปรันจริง)
# ==========================================
FROM debian:bookworm-slim

WORKDIR /app

# ติดตั้งเฉพาะ library ที่จำเป็นตอนรัน (libpq5 สำหรับ Postgres และ ca-certificates สำหรับยิง HTTPS)
RUN apt-get update \
    && apt-get install -y --no-install-recommends libpq5 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# คัดลอกเฉพาะไฟล์ Binary (ที่ Build เสร็จแล้ว) จาก Stage 1 มาที่ Stage 2
# เปลี่ยนชื่อ rust-core-service เป็นชื่อ package ใน Cargo.toml ของคุณ
COPY --from=builder /usr/src/app/target/release/rust-core-service /app/rust-core-service

# เปิด Port (ให้ตรงกับที่แอปคุณรัน)
EXPOSE 8080

# คำสั่งสำหรับรันแอป
CMD ["./rust-core-service"]
