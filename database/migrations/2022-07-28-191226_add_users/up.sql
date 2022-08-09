-- Your SQL goes here

ALTER TABLE products
DROP COLUMN notification;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL
);

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
	  REFERENCES users(id)
	  ON DELETE CASCADE
);

CREATE TABLE notifications (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,
    CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
	  REFERENCES users(id)
	  ON DELETE CASCADE,
    CONSTRAINT fk_product
      FOREIGN KEY(product_id) 
	  REFERENCES products(id)
	  ON DELETE CASCADE
);

CREATE TABLE collections (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    user_id INTEGER NOT NULL,
    CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
	  REFERENCES users(id)
	  ON DELETE CASCADE
);

CREATE TABLE collections_products_relation(
    id SERIAL PRIMARY KEY,
    collection_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,
    CONSTRAINT fk_collection
      FOREIGN KEY(collection_id)
	  REFERENCES collections(id)
	  ON DELETE CASCADE,
    CONSTRAINT fk_product
      FOREIGN KEY(product_id) 
	  REFERENCES products(id)
	  ON DELETE CASCADE
);

ALTER TABLE products
ADD COLUMN description TEXT;
