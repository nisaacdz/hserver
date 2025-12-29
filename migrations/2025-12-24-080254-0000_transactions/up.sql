-- Your SQL goes here

CREATE TYPE transaction_status AS ENUM ('empty', 'pending', 'failed', 'succeeded', 'reversed');
CREATE TYPE transaction_kind AS ENUM ('incoming', 'outgoing');

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    booking_id UUID NOT NULL REFERENCES bookings(block_id), -- Links to your existing PK
    external_id TEXT NOT NULL, -- The Stripe/PayPal Intent ID
    amount DECIMAL(10, 2) NOT NULL,
    currency TEXT NOT NULL,
    status transaction_status NOT NULL,
    kind transaction_kind NOT NULL,
    label TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_transactions_external_id ON transactions(external_id);

CREATE TRIGGER update_transactions_modtime
BEFORE UPDATE ON transactions
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
