-- Your SQL goes here

-- Table: transportation_expenses (บันทึกประวัติค่าใช้จ่ายในการเดินทาง)
-- amount เก็บหน่วยเป็นสตางค์เหมือน transactions.amount เพื่อให้บวกลบไม่มีปัญหาปัดเศษ
-- ผูกกับ transactions ผ่าน transaction_id (nullable) สำหรับตอนที่ frontend ติ๊ก
-- "บันทึกลงรายการรายจ่ายด้วย" จะได้รู้ว่ารายการไหน sync ไป transactions แล้วบ้าง
-- ป้องกันการนับค่าใช้จ่ายซ้ำตอนรวม stats จากทั้งสองตาราง
CREATE TABLE transportation_expenses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    transaction_id UUID REFERENCES transactions(id) ON DELETE SET NULL,
    -- category เก็บเป็น value ของ property_options (property_type: TRAVEL_EXPENSES)
    -- เช่น refuel, sky_train, mrt, bus, grab ใช้ทำ stats แยกตามหมวด (ค่าน้ำมันรวม, ค่ารถไฟฟ้ารวม ฯลฯ)
    category VARCHAR(50) NOT NULL,
    amount BIGINT NOT NULL,
    title VARCHAR(100) NOT NULL,
    note VARCHAR(3000),
    expense_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_transportation_expense_amount_positive CHECK (amount > 0),
    CONSTRAINT uq_transportation_expense_transaction_id UNIQUE (transaction_id)
);

-- หน้า list: รายการล่าสุดของ user
CREATE INDEX idx_transportation_expenses_user_date ON transportation_expenses(user_id, expense_date DESC);
-- Stats: ค่าเดินทางรวม / แยกตามหมวดหมู่ (น้ำมัน, รถไฟฟ้า, รถสาธารณะ ฯลฯ) รายเดือน-รายปี
CREATE INDEX idx_transportation_expenses_user_category_date ON transportation_expenses(user_id, category, expense_date);
