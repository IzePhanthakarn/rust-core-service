use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand_core::OsRng;

// ฟังก์ชันสำหรับ Hash รหัสผ่านก่อนลง Database
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // คืนค่าเป็น String ที่เข้ารหัสแล้ว (เช่น $argon2id$v=19$...)
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

// ฟังก์ชันสำหรับตรวจสอบรหัสผ่านตอน Login
pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };

    // เทียบรหัสผ่านที่รับมากับ Hash ใน DB
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*; // ดึงฟังก์ชันทั้งหมดในไฟล์นี้มาใช้

    #[test] // บอก Rust ว่านี่คือฟังก์ชันเทสต์นะ
    fn test_password_hashing_and_verification() {
        // 1. จำลองข้อมูล
        let password = "my_super_secret_password";

        // 2. ทดสอบการ Hash (ต้องไม่เกิด Error)
        let hash = hash_password(password).expect("ควรจะเข้ารหัสผ่านได้");

        // รหัสผ่านที่ Hash แล้วต้องหน้าตาไม่เหมือนรหัสผ่านต้นฉบับ
        assert_ne!(password, hash);

        // 3. ทดสอบการ Verify รหัสผ่านที่ถูกต้อง (ต้องเป็น true)
        let is_valid = verify_password(&hash, password);
        assert!(is_valid, "รหัสผ่านที่ถูกต้อง ควรจะ verify ผ่าน");

        // 4. ทดสอบการ Verify รหัสผ่านที่ผิด (ต้องเป็น false)
        let is_invalid = verify_password(&hash, "wrong_password");
        assert!(!is_invalid, "รหัสผ่านที่ผิด ต้อง verify ไม่ผ่าน");
    }
}
