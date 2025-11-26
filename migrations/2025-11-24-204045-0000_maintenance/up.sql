-- Your SQL goes here

CREATE TYPE "maintenance_kind" AS ENUM (
    'electrical',
    'plumbing',
    'structural',
    'hvac',
    'fire_safety',
    'security_systems',
    'groundskeeping',
    'janitorial',
    'pest_control',
    'it_network',
    'painting',
    'appliances',
    'other'
);

CREATE TYPE "maintenance_severity" AS ENUM('low', 'medium', 'high');

CREATE TABLE maintenance (
    block_id UUID PRIMARY KEY REFERENCES blocks(id) ON DELETE CASCADE,
    kind "maintenance_kind" NOT NULL,
    severity "maintenance_severity" NOT NULL,
);
