-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS idx_transportation_expenses_user_category_date;
DROP INDEX IF EXISTS idx_transportation_expenses_user_date;
DROP TABLE IF EXISTS transportation_expenses;
