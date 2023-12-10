CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE beers (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    cost BIGINT NOT NULL,
    created_at BIGINT DEFAULT CAST(EXTRACT(EPOCH FROM NOW()) AS INTEGER),
    updated_at BIGINT DEFAULT CAST(EXTRACT(EPOCH FROM NOW()) AS INTEGER)
);