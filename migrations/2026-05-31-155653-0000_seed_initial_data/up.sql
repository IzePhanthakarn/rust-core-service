-- 1. Insert Super Admin User
-- หมายเหตุ: password_hash ด้านล่างคือคำว่า 'P@ssw0rd' ที่ผ่านการ Hash ด้วย Argon2
INSERT INTO users (id, email, secret_word, password_hash, role, status) 
VALUES (
    'c8adb331-dcf6-47ad-ad15-066a145127b3', -- Fix ID เพื่อให้จัดการง่ายตอน Seed
    'admin@mail.com', 
    '$argon2id$v=19$m=19456,t=2,p=1$9GG5+RyHBuggbhJd32QqGQ$2pSEASAa0IWgaGkkYK7CiTDTMECyyYeHmwJdbHCXXrc', -- Jum1n0@1357
    '$argon2id$v=19$m=19456,t=2,p=1$euZEQcAj1WztSn7tp5/E+Q$XrMOStEyPVp5lktJwA7TbPitBVZ274SnXUtKtWb1co0', -- P@ssw0rd
    'super_admin',
    'active'
);

-- 4. Create Profile for Super Admin
INSERT INTO user_profiles (user_id, first_name, last_name)
VALUES ('c8adb331-dcf6-47ad-ad15-066a145127b3', 'Admin', 'Core');