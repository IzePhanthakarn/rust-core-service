-- Your SQL goes here

-- Table: transportation_expenses (records travel expense history)
-- amount is stored in satang (cents), same as transactions.amount, so addition/subtraction has no rounding issues
-- linked to transactions via transaction_id (nullable), for when the frontend checks
-- "also record this as an expense transaction" so we know which entries have already synced to transactions
-- this prevents double-counting expenses when aggregating stats from both tables
CREATE TABLE transportation_expenses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    transaction_id UUID REFERENCES transactions(id) ON DELETE SET NULL,
    -- category stores the value from property_options (property_type: TRAVEL_EXPENSES)
    -- e.g. refuel, sky_train, mrt, bus, grab; used for stats broken down by category (total fuel cost, total train cost, etc.)
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

-- List page: user's most recent entries
CREATE INDEX idx_transportation_expenses_user_date ON transportation_expenses(user_id, expense_date DESC);
-- Stats: total travel expenses / broken down by category (fuel, train, public transport, etc.) monthly-yearly
CREATE INDEX idx_transportation_expenses_user_category_date ON transportation_expenses(user_id, category, expense_date);
