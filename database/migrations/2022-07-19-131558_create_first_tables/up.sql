-- Your SQL goes here

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE offers (
    id SERIAL PRIMARY KEY,
    url TEXT NOT NULL UNIQUE,
    product_id INTEGER NOT NULL,
    CONSTRAINT fk_product
        FOREIGN KEY(product_id) 
        REFERENCES products(id)
        ON DELETE CASCADE
);

CREATE TABLE prices (
    id SERIAL PRIMARY KEY,
    offer_id INTEGER NOT NULL,
    value FLOAT8 NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_offer
      FOREIGN KEY(offer_id) 
	  REFERENCES offers(id)
	  ON DELETE CASCADE
);

