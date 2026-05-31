DELETE FROM users WHERE email = 'admin@rustcore.local';
DELETE FROM roles WHERE name IN ('super_admin', 'admin_roles', 'user');