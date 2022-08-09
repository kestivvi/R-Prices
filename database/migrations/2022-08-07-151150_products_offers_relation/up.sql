-- Your SQL goes here

CREATE TABLE IF NOT EXISTS products_offers_relation (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL,
    offer_id INTEGER NOT NULL,
    CONSTRAINT fk_product
      FOREIGN KEY(product_id) 
	  REFERENCES products(id)
	  ON DELETE CASCADE,
    CONSTRAINT fk_offer
      FOREIGN KEY(offer_id)
	  REFERENCES offers(id)
	  ON DELETE CASCADE
);

INSERT INTO products_offers_relation (product_id, offer_id)
SELECT product_id, id FROM offers
WHERE NOT EXISTS (SELECT * FROM products_offers_relation);

ALTER TABLE offers
DROP COLUMN IF EXISTS product_id;
