-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS transactions;

DROP TYPE IF EXISTS transaction_status;
DROP TYPE IF EXISTS transaction_kind;

