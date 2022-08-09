-- This file should undo anything in `up.sql`

ALTER TABLE products
ADD COLUMN notification BOOLEAN NOT NULL DEFAULT true;

DROP TABLE IF EXISTS collections_products_relation;
DROP TABLE IF EXISTS collections;
DROP TABLE IF EXISTS notifications;
DROP TABLE IF EXISTS sessions;
DROP TABLE IF EXISTS users;

ALTER TABLE products
DROP COLUMN description;
