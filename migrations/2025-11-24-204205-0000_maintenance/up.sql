-- Your SQL goes here

CREATE TYPE "maintenance_severity" AS ENUM('low', 'medium', 'high');

CREATE TABLE maintenance (
    block_id UUID PRIMARY KEY REFERENCES blocks(id) ON DELETE CASCADE,
    kind "maintenance_kind" NOT NULL,
    severity "maintenance_severity" NOT NULL,
);
