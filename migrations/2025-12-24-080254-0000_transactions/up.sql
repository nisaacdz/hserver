-- Your SQL goes here

CREATE TYPE transaction_status AS ENUM ('pending', 'failed', 'succeeded');
CREATE TYPE external_provider AS ENUM ('stripe', 'paystack');

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reservation_id UUID NOT NULL REFERENCES reservations(id) ON DELETE CASCADE,
    external_provider external_provider NOT NULL,
    external_id TEXT NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    currency TEXT NOT NULL,
    status transaction_status NOT NULL,
    label TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_transactions_external_id ON transactions(external_id);

CREATE TRIGGER update_transactions_modtime
BEFORE UPDATE ON transactions
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
