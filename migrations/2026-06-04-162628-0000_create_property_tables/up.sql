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
('155deadd-e1b4-4296-bf5c-0b4e27728438','Productivity Score', 'PRODUCTIVITY_SCORE', 'Score for productivity', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Productivity Score Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES 
('155deadd-e1b4-4296-bf5c-0b4e27728438', 1, '🐢 Low Productivity', '1', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('155deadd-e1b4-4296-bf5c-0b4e27728438', 2, '🧩 Below Average', '2', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('155deadd-e1b4-4296-bf5c-0b4e27728438', 3, '⏳ Standard', '3', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('155deadd-e1b4-4296-bf5c-0b4e27728438', 4, '⚡ Good Flow', '4', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('155deadd-e1b4-4296-bf5c-0b4e27728438', 5, '🧠 Peak Performance', '5', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');
-- Value: 1 | Label: " 🐢Low Productivity" (or "Struggling")
-- Value: 2 | Label: "🧩 Below Average" (or "Distracted")
-- Value: 3 | Label: "⏳ Standard" (or "Steady")
-- Value: 4 | Label: "⚡ Good Flow" (or "Productive")
-- Value: 5 | Label: "🧠 Peak Performance" (or "Deep Work")

-- Insert Mood Score Property Types
INSERT INTO property_types (id, name, code, description, created_by, updated_by) VALUES 
('c89b46ba-f696-4822-9593-7da3e7cc94e0','Mood Score', 'MOOD_SCORE', 'Score for mood', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Mood Score Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES 
('c89b46ba-f696-4822-9593-7da3e7cc94e0', 1, '🌪️ Challenging', '1', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('c89b46ba-f696-4822-9593-7da3e7cc94e0', 2, '☁️ Subpar', '2', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('c89b46ba-f696-4822-9593-7da3e7cc94e0', 3, '😐 Neutral', '3', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('c89b46ba-f696-4822-9593-7da3e7cc94e0', 4, '☀️ Positive', '4', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('c89b46ba-f696-4822-9593-7da3e7cc94e0', 5, '🎉 Excellent', '5', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');
-- 1: 🌪️ Challenging (a day you feel exhausted or run into a lot of problems)
-- 2: ☁️ Subpar (a day you're not in a great mood or lack motivation)
-- 3: 😐 Neutral (a normal, ordinary day)
-- 4: ☀️ Positive (a day you feel good and energetic)
-- 5: 🎉 Excellent (a day you feel accomplished and very happy)

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

-- Insert Transaction Type Property Types
INSERT INTO property_types (id, name, code, description, created_by, updated_by) VALUES
('2f0d7ee2-6d0e-4c47-9d02-2fbec3b7c1a3','Transaction Type', 'TRANSACTION_TYPE', 'Property type for transaction type', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Transaction Type Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES
('2f0d7ee2-6d0e-4c47-9d02-2fbec3b7c1a3', 1, '💰 Income', 'income', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('2f0d7ee2-6d0e-4c47-9d02-2fbec3b7c1a3', 2, '💸 Expense', 'expense', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Transaction Expense Category Property Types
INSERT INTO property_types (id, name, code, description, created_by, updated_by) VALUES
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f','Transaction Expense Category', 'TRANSACTION_EXPENSE_CATEGORY', 'Category options for expense transactions', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Transaction Expense Category Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 1, '🍜 Food & Drinks', 'food', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 2, '🚗 Transport', 'transport', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 3, '🏠 Housing', 'housing', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 4, '🧾 Bills & Utilities', 'bills', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 5, '🛒 Shopping', 'shopping', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 6, '🎬 Entertainment', 'entertainment', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 7, '🔁 Subscriptions', 'subscription', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 8, '🏥 Health', 'health', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 9, '📚 Education', 'education', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 10, '✈️ Travel', 'travel', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 11, '🛡️ Insurance & Tax', 'insurance_tax', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 12, '💳 Debt & Loan', 'debt', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 13, '🐶 Pet', 'pet', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 14, '🎁 Gift & Donation', 'gift_donation', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 15, '📈 Saving & Investment', 'saving_investment', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('4c1a4b8e-7c6a-4b1e-9f4d-1a2b3c4d5e6f', 16, '📦 Other', 'other_expense', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Transaction Income Category Property Types
INSERT INTO property_types (id, name, code, description, created_by, updated_by) VALUES
('9b3f5d21-8e4c-4a77-b0d5-6e7f8a9b0c1d','Transaction Income Category', 'TRANSACTION_INCOME_CATEGORY', 'Category options for income transactions', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Transaction Income Category Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES
('9b3f5d21-8e4c-4a77-b0d5-6e7f8a9b0c1d', 1, '💼 Salary', 'salary', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('9b3f5d21-8e4c-4a77-b0d5-6e7f8a9b0c1d', 2, '🎉 Bonus', 'bonus', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('9b3f5d21-8e4c-4a77-b0d5-6e7f8a9b0c1d', 3, '💻 Freelance', 'freelance', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('9b3f5d21-8e4c-4a77-b0d5-6e7f8a9b0c1d', 4, '📈 Investment Return', 'investment_return', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('9b3f5d21-8e4c-4a77-b0d5-6e7f8a9b0c1d', 5, '🏪 Sale', 'sale', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('9b3f5d21-8e4c-4a77-b0d5-6e7f8a9b0c1d', 6, '🎁 Gift Received', 'gift_received', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('9b3f5d21-8e4c-4a77-b0d5-6e7f8a9b0c1d', 7, '↩️ Refund', 'refund', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('9b3f5d21-8e4c-4a77-b0d5-6e7f8a9b0c1d', 8, '📦 Other', 'other_income', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Billing Cycle Property Types
INSERT INTO property_types (id, name, code, description, created_by, updated_by) VALUES
('d5e6f7a8-9b0c-4d1e-8f2a-3b4c5d6e7f80','Billing Cycle', 'BILLING_CYCLE', 'Billing cycle options for subscriptions', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Billing Cycle Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES
('d5e6f7a8-9b0c-4d1e-8f2a-3b4c5d6e7f80', 1, 'Monthly', 'monthly', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('d5e6f7a8-9b0c-4d1e-8f2a-3b4c5d6e7f80', 2, 'Yearly', 'yearly', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Status Property Types
INSERT INTO property_types (id, name, code, description, created_by, updated_by) VALUES
('e1f2a3b4-c5d6-4e7f-9a0b-1c2d3e4f5a6b','Status', 'STATUS', 'Generic active / inactive status options', 'c8adb331-dcf6-47ad-ad15-066a145127b3', 'c8adb331-dcf6-47ad-ad15-066a145127b3');

-- Insert Status Property Options
INSERT INTO property_options (property_type_id, sort_order, label, value, is_active, created_by) VALUES
('e1f2a3b4-c5d6-4e7f-9a0b-1c2d3e4f5a6b', 1, 'Active', 'active', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3'),
('e1f2a3b4-c5d6-4e7f-9a0b-1c2d3e4f5a6b', 2, 'Inactive', 'inactive', true, 'c8adb331-dcf6-47ad-ad15-066a145127b3');
