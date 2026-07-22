-- Your SQL goes here

-- ===== Custom Types (Enums) =====
CREATE TYPE transaction_type AS ENUM ('income', 'expense');
CREATE TYPE billing_cycle AS ENUM ('monthly', 'yearly');

-- 1. Table: transactions (records income and expenses)
--    amount is stored in satang (cents) and is always positive; the direction of money is determined by type
--    (e.g. 100.50 THB = 10050) so addition/subtraction has no rounding issues, and it maps directly to i64
--    category stores the value from property_options (property_type: TRANSACTION_CATEGORY)
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

-- List page: user's most recent transactions
CREATE INDEX idx_transactions_user_date ON transactions(user_id, transaction_date DESC);
-- Stats: monthly/yearly income & expense summary
CREATE INDEX idx_transactions_user_type_date ON transactions(user_id, type, transaction_date);
-- Stats: totals broken down by category (pie chart / top spending)
CREATE INDEX idx_transactions_user_category_date ON transactions(user_id, category, transaction_date);

-- 2. Table: subscriptions (recurring expenses e.g. Netflix, Spotify, internet bill, yearly domain fee)
--    Kept completely separate from transactions, does not interfere with the income/expense recording flow
CREATE TABLE subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    -- unit is satang (cents), same as transactions.amount
    amount BIGINT NOT NULL,
    billing_cycle billing_cycle NOT NULL DEFAULT 'monthly',
    -- billing day (1-31), shared by both monthly and yearly cycles
    billing_day INTEGER NOT NULL,
    -- billing month (1-12), used only for yearly cycle; must be NULL for monthly
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
-- Find subscriptions billing this month (every monthly one + yearly ones matching this month)
CREATE INDEX idx_subscriptions_user_cycle ON subscriptions(user_id, billing_cycle, billing_month) WHERE is_active;
