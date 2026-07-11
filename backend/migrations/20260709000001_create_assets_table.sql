-- Add migration script here
CREATE TABLE assets (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  symbol VARCHAR(20) NOT NULL UNIQUE,
  name VARCHAR(255) NOT NULL,
  asset_type VARCHAR(50) NOT NULL,
  exchange VARCHAR(50),
  currency VARCHAR(10) NOT NULL DEFAULT 'USD',
  current_price DOUBLE PRECISION NOT NULL DEFAULT 0.0,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);