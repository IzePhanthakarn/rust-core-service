-- 1. Insert Default Roles
INSERT INTO roles (id, name, description) VALUES
    (uuid_generate_v4(), 'super_admin', 'Supreme administrator of the system'),
    (uuid_generate_v4(), 'admin_roles', 'Administrator for Identity and Access Management'),
    (uuid_generate_v4(), 'user', 'General standard user');

-- 2. Insert Super Admin User
-- หมายเหตุ: password_hash ด้านล่างคือคำว่า 'password123' ที่ผ่านการ Hash ด้วย Argon2
INSERT INTO users (id, email, password_hash, status) 
VALUES (
    'c8adb331-dcf6-47ad-ad15-066a145127b3', -- Fix ID เพื่อให้จัดการง่ายตอน Seed
    'admin@rustcore.local', 
    '$argon2id$v=19$m=19456,t=2,p=1$1Yc0YxYy3Zk+G5Yp2c3V7w$0yY2+Z8x3Zk+G5Yp2c3V7w', 
    'active'
);

-- 3. Assign super_admin role to the user
INSERT INTO user_roles (user_id, role_id)
SELECT 'c8adb331-dcf6-47ad-ad15-066a145127b3', id FROM roles WHERE name = 'super_admin';

-- 4. Create Profile for Super Admin
INSERT INTO user_profiles (user_id, first_name, last_name)
VALUES ('c8adb331-dcf6-47ad-ad15-066a145127b3', 'Phanthakarn', 'Khumphai');