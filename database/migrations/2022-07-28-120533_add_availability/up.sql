-- Your SQL goes here

CREATE TYPE availability AS ENUM (
    'available',
    'temporarily_unavailable',
    'unavailable',
    'price_not_found',
    'site_not_found'
);

ALTER TABLE prices
ADD COLUMN availability availability NOT NULL DEFAULT 'available';
