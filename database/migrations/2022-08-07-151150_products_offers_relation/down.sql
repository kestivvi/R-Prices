-- This file should undo anything in `up.sql`

ALTER TABLE offers
ADD COLUMN IF NOT EXISTS product_id INTEGER;

UPDATE offers
SET product_id = relation.product_id
FROM (
    SELECT product_id, offer_id FROM products_offers_relation
) AS relation
WHERE offers.id = relation.offer_id;

DROP TABLE IF EXISTS products_offers_relation;
