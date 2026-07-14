-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS subscriptions;
DROP TABLE IF EXISTS transactions;

DROP TYPE IF EXISTS billing_cycle;
DROP TYPE IF EXISTS transaction_type;
