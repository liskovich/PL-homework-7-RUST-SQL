CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS oil_platforms (
  id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
  platform_type VARCHAR(255) NOT NULL,
  platform_level SMALLINT NOT NULL DEFAULT 0,
  profitability BIGINT NOT NULL DEFAULT 10,
  created_at BIGINT DEFAULT CAST(EXTRACT(EPOCH FROM NOW()) AS INTEGER),
  updated_at BIGINT DEFAULT CAST(EXTRACT(EPOCH FROM NOW()) AS INTEGER)
);

INSERT INTO oil_platforms (id, platform_type, platform_level, profitability)
VALUES
  ('a63169d5-4107-4a0a-9d82-dad11b6500b3', 'Rig', 0, 10),
  ('be251379-53fe-4e9d-a80e-caf59a388123', 'Ground', 1, 20),
  ('226d0256-e280-4262-ab57-b1e30129805d', 'Pump', 1, 20),
  ('fb4bcfa2-dd9b-4b17-95b1-f5bebf9d6e30', 'Rig', 10, 100);