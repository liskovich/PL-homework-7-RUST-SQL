CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE oil_platforms_upgrades (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    platform_id UUID NOT NULL,
    new_platform_level SMALLINT NOT NULL,
    profitability_addition BIGINT NOT NULL,
    created_at BIGINT DEFAULT CAST(EXTRACT(EPOCH FROM NOW()) AS INTEGER),
    updated_at BIGINT DEFAULT CAST(EXTRACT(EPOCH FROM NOW()) AS INTEGER),
    -- Define foreign key constraint
    FOREIGN KEY (platform_id) REFERENCES oil_platforms(id)
);