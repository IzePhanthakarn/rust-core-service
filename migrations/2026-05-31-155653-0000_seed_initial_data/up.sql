-- 1. Insert Default Roles
INSERT INTO roles (id, name, description) VALUES
    ('ea56043a-abf7-4b08-808d-c04acc41850c', 'super_admin', 'Supreme administrator of the system'),
    ('38097a11-9ea3-443e-b7df-477826e7d348', 'admin_roles', 'Administrator for Identity and Access Management'),
    ('1e67fab9-184c-4563-b0e5-c78053a59e95', 'user', 'General standard user');

-- 2. Insert Super Admin User
-- หมายเหตุ: password_hash ด้านล่างคือคำว่า 'P@ssw0rd' ที่ผ่านการ Hash ด้วย Argon2
INSERT INTO users (id, email, secret_word, password_hash, status) 
VALUES (
    'c8adb331-dcf6-47ad-ad15-066a145127b3', -- Fix ID เพื่อให้จัดการง่ายตอน Seed
    'admin@mail.com', 
    '$argon2id$v=19$m=19456,t=2,p=1$9GG5+RyHBuggbhJd32QqGQ$2pSEASAa0IWgaGkkYK7CiTDTMECyyYeHmwJdbHCXXrc', -- Jum1n0@1357
    '$argon2id$v=19$m=19456,t=2,p=1$euZEQcAj1WztSn7tp5/E+Q$XrMOStEyPVp5lktJwA7TbPitBVZ274SnXUtKtWb1co0', -- P@ssw0rd
    'active'
);

-- 3. Assign super_admin role to the user
INSERT INTO user_roles (user_id, role_id) VALUES 
    ('c8adb331-dcf6-47ad-ad15-066a145127b3', 'ea56043a-abf7-4b08-808d-c04acc41850c'),
    ('38097a11-9ea3-443e-b7df-477826e7d348', 'ea56043a-abf7-4b08-808d-c04acc41850c');
-- 4. Create Profile for Super Admin
INSERT INTO user_profiles (user_id, first_name, last_name)
VALUES ('c8adb331-dcf6-47ad-ad15-066a145127b3', 'Admin', 'Core');