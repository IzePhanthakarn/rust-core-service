CREATE TABLE property_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    code VARCHAR(50) NOT NULL UNIQUE,
    description VARCHAR(255),
    created_by UUID NOT NULL REFERENCES users(id),
    updated_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_property_types_name ON property_types(name);

CREATE TABLE property_options (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    property_type_id UUID NOT NULL REFERENCES property_types(id) ON DELETE CASCADE,
    sort_order INTEGER NOT NULL,
    label VARCHAR(100) NOT NULL,
    value VARCHAR(50) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_property_value UNIQUE (property_type_id, value)
);

CREATE INDEX idx_property_options_type_sort ON property_options(property_type_id, sort_order);

-- Insert Travel Expenses Property Types
INSERT INTO property_types (id, name, code, description, created_by, updated_by) VALUES 
('17980cc9-6393-4b5c-b226-fcab774b539e','Travel Expenses', 'TRAVEL_EXPENSES', 'Property type for travel expenses', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Travel Expenses Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES 
('17980cc9-6393-4b5c-b226-fcab774b539e', 1, 'Refuel', 'refuel', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('17980cc9-6393-4b5c-b226-fcab774b539e', 2, 'Sky Train', 'sky_train', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('17980cc9-6393-4b5c-b226-fcab774b539e', 2, 'Ride Hailing', 'ride_hailing', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Work Tags Property Types
INSERT INTO property_types (id, name, code, description, created_by, updated_by) VALUES 
('08f3aeaf-2a8d-458c-a212-2106edfced8f','Work Tags', 'WORK_TAGS', 'Property type for work tags', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Work Tags Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES 
('08f3aeaf-2a8d-458c-a212-2106edfced8f', 1, 'Meeting', 'meeting', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('08f3aeaf-2a8d-458c-a212-2106edfced8f', 2, 'Coding', 'coding', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('08f3aeaf-2a8d-458c-a212-2106edfced8f', 3, 'Research', 'research', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Productivity Score Property Types
INSERT INTO property_types (id, name, code, description, created_by, updated_by) VALUES 
('155deadd-e1b4-4296-bf5c-0b4e27728438','Productivity Score', 'PRODUCTIVITY_SCORE', 'score for productivity', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Productivity Score Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES 
('155deadd-e1b4-4296-bf5c-0b4e27728438', 1, '🐢 Low Productivity', '1', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('155deadd-e1b4-4296-bf5c-0b4e27728438', 2, '🧩 Below Average', '2', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('155deadd-e1b4-4296-bf5c-0b4e27728438', 3, '⏳ Standard', '3', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('155deadd-e1b4-4296-bf5c-0b4e27728438', 4, '⚡ Good Flow', '4', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('155deadd-e1b4-4296-bf5c-0b4e27728438', 5, '🧠 Peak Performance', '5', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');
-- Value: 1 | Label: " 🐢Low Productivity" (หรือ "Struggling")
-- Value: 2 | Label: "🧩 Below Average" (หรือ "Distracted")
-- Value: 3 | Label: "⏳ Standard" (หรือ "Steady")
-- Value: 4 | Label: "⚡ Good Flow" (หรือ "Productive")
-- Value: 5 | Label: "🧠 Peak Performance" (หรือ "Deep Work")

-- Insert Mood Score Property Types
INSERT INTO property_types (id, name, code, description, created_by, updated_by) VALUES 
('c89b46ba-f696-4822-9593-7da3e7cc94e0','Mood Score', 'MOOD_SCORE', 'score for mood', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Mood Score Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES 
('c89b46ba-f696-4822-9593-7da3e7cc94e0', 1, '🌪️ Challenging', '1', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('c89b46ba-f696-4822-9593-7da3e7cc94e0', 2, '☁️ Subpar', '2', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('c89b46ba-f696-4822-9593-7da3e7cc94e0', 3, '😐 Neutral', '3', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('c89b46ba-f696-4822-9593-7da3e7cc94e0', 4, '☀️ Positive', '4', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('c89b46ba-f696-4822-9593-7da3e7cc94e0', 5, '🎉 Excellent', '5', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');
-- 1: 🌪️ Challenging (วันที่รู้สึกเหนื่อยล้า หรือมีปัญหาเข้ามามาก)
-- 2: ☁️ Subpar (วันที่อารมณ์ไม่ค่อยดี หรือไม่ค่อยมีแรงบันดาลใจ)
-- 3: 😐 Neutral (วันที่ปกติ ทั่วไป)
-- 4: ☀️ Positive (วันที่รู้สึกดี มีพลังงาน)
-- 5: 🎉 Excellent (วันที่รู้สึกถึงความสำเร็จ และมีความสุขมาก)

-- Insert Month Property Types
INSERT INTO property_types (id, name, code, description, created_by, updated_by) VALUES 
('b7a4db1f-dbf4-44e3-b769-ba5482ed3ec6','Month', 'MONTH', '12 Month Property type', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Month Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES 
('b7a4db1f-dbf4-44e3-b769-ba5482ed3ec6', 1, 'January', '01', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('b7a4db1f-dbf4-44e3-b769-ba5482ed3ec6', 2, 'February', '02', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('b7a4db1f-dbf4-44e3-b769-ba5482ed3ec6', 2, 'March', '03', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('b7a4db1f-dbf4-44e3-b769-ba5482ed3ec6', 2, 'April', '04', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('b7a4db1f-dbf4-44e3-b769-ba5482ed3ec6', 2, 'May', '05', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('b7a4db1f-dbf4-44e3-b769-ba5482ed3ec6', 2, 'June', '06', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('b7a4db1f-dbf4-44e3-b769-ba5482ed3ec6', 2, 'July', '07', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('b7a4db1f-dbf4-44e3-b769-ba5482ed3ec6', 2, 'August', '08', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('b7a4db1f-dbf4-44e3-b769-ba5482ed3ec6', 2, 'September', '09', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('b7a4db1f-dbf4-44e3-b769-ba5482ed3ec6', 2, 'October', '10', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('b7a4db1f-dbf4-44e3-b769-ba5482ed3ec6', 2, 'November', '11', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('b7a4db1f-dbf4-44e3-b769-ba5482ed3ec6', 2, 'December', '12', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Year Property Types
INSERT INTO property_types (id, name, code, description, created_by, updated_by) VALUES 
('ea1f0a40-8244-48d5-a6a6-0ea9e4eeb962','Year', 'YEAR', 'Year Property type', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Year Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES 
('ea1f0a40-8244-48d5-a6a6-0ea9e4eeb962', 1, '2026', '2026', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('ea1f0a40-8244-48d5-a6a6-0ea9e4eeb962', 2, '2027', '2027', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Event Property Types
INSERT INTO property_types (id, name, code, description, created_by, updated_by) VALUES 
('adfddf5f-1639-4477-9330-1bf8e569ad7a','Event', 'EVENT', 'Property type for event', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Event Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES 
('adfddf5f-1639-4477-9330-1bf8e569ad7a', 1, 'Meeting', 'blue', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('adfddf5f-1639-4477-9330-1bf8e569ad7a', 2, 'Personal Leave', 'gray', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('adfddf5f-1639-4477-9330-1bf8e569ad7a', 3, 'Onsite Travel', 'mint', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('adfddf5f-1639-4477-9330-1bf8e569ad7a', 4, 'Deployment', 'crimson', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('adfddf5f-1639-4477-9330-1bf8e569ad7a', 5, 'Code Review', 'purple', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('adfddf5f-1639-4477-9330-1bf8e569ad7a', 6, 'Deep Work', 'terracotta', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('adfddf5f-1639-4477-9330-1bf8e569ad7a', 7, 'Server Maintenance', 'amber', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('adfddf5f-1639-4477-9330-1bf8e569ad7a', 8, 'Public Holidays', 'coral', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');
