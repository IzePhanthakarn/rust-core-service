-- Your SQL goes here

-- ===== Custom Types (Enums) =====
CREATE TYPE transaction_type AS ENUM ('income', 'expense');
CREATE TYPE billing_cycle AS ENUM ('monthly', 'yearly');

-- 1. Table: transactions (บันทึกรายรับ-รายจ่าย)
--    amount เก็บหน่วยเป็นสตางค์ และเป็นค่าบวกเสมอ ทิศทางของเงินดูจาก type
--    (เช่น 100.50 บาท = 10050) เพื่อให้บวกลบไม่มีปัญหาปัดเศษ และ map เป็น i64 ตรงๆ
--    category เก็บเป็น value ของ property_options (property_type: TRANSACTION_CATEGORY)
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type transaction_type NOT NULL,
    amount BIGINT NOT NULL,
    category VARCHAR(50) NOT NULL,
    title VARCHAR(100) NOT NULL,
    note VARCHAR(3000),
    transaction_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_transaction_amount_positive CHECK (amount > 0)
);

-- หน้า list: รายการล่าสุดของ user
CREATE INDEX idx_transactions_user_date ON transactions(user_id, transaction_date DESC);
-- Stats: สรุปรายรับ/รายจ่าย รายเดือน-รายปี
CREATE INDEX idx_transactions_user_type_date ON transactions(user_id, type, transaction_date);
-- Stats: ยอดแยกตามหมวดหมู่ (pie chart / top spending)
CREATE INDEX idx_transactions_user_category_date ON transactions(user_id, category, transaction_date);

-- 2. Table: subscriptions (ค่าใช้จ่ายประจำ เช่น Netflix, Spotify, ค่าเน็ต, ค่าโดเมนรายปี)
--    แยกขาดจาก transactions ไม่ยุ่งกับ flow การบันทึกรายรับรายจ่าย
CREATE TABLE subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    -- หน่วยเป็นสตางค์เหมือน transactions.amount
    amount BIGINT NOT NULL,
    billing_cycle billing_cycle NOT NULL DEFAULT 'monthly',
    -- วันที่ตัดเงิน (1-31) ใช้ร่วมกันทั้งรายเดือนและรายปี
    billing_day INTEGER NOT NULL,
    -- เดือนที่ตัดเงิน (1-12) ใช้เฉพาะรายปี ถ้าเป็นรายเดือนต้องเป็น NULL
    billing_month INTEGER,
    category VARCHAR(50),
    note VARCHAR(3000),
    start_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_date TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_subscription_amount_positive CHECK (amount > 0),
    CONSTRAINT chk_subscription_billing_day CHECK (billing_day BETWEEN 1 AND 31),
    CONSTRAINT chk_subscription_billing_month CHECK (
        (billing_cycle = 'yearly' AND billing_month IS NOT NULL AND billing_month BETWEEN 1 AND 12)
        OR
        (billing_cycle = 'monthly' AND billing_month IS NULL)
    )
);

CREATE INDEX idx_subscriptions_user_id ON subscriptions(user_id);
-- หา subscription ที่จะตัดเงินในเดือนนี้ (รายเดือนทุกตัว + รายปีที่ตรงเดือน)
CREATE INDEX idx_subscriptions_user_cycle ON subscriptions(user_id, billing_cycle, billing_month) WHERE is_active;
