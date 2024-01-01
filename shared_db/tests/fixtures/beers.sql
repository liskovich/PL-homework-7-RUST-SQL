CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS beers (
  id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
  title VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  thumbnail TEXT NOT NULL,
  cost BIGINT NOT NULL,
  purchased BOOLEAN DEFAULT FALSE,
  created_at BIGINT DEFAULT CAST(EXTRACT(EPOCH FROM NOW()) AS INTEGER),
  updated_at BIGINT DEFAULT CAST(EXTRACT(EPOCH FROM NOW()) AS INTEGER)
);

INSERT INTO beers (id, title, description, thumbnail, cost, purchased)
VALUES 
  ('8983e5a3-f6b5-40c5-81a2-c02a498116fc', 'Beer 1', 'Description 1', 'img_1.png', 5, TRUE),
  ('effaf31e-aedc-42de-adce-322d73ef69d0', 'Beer 2', 'Description 2', 'img_2.png', 10, FALSE),
  ('bb0c549e-1a3e-4c16-bb1f-37f6b6dce551', 'Beer 3', 'Description 3', 'img_3.png', 15, FALSE),
  ('15d114d0-4ca6-4f9e-ae73-c799253d5001', 'Beer 4', 'Description 4', 'img_4.png', 20, FALSE);