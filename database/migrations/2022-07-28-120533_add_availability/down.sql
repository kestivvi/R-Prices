-- This file should undo anything in `up.sql`

ALTER TABLE prices
DROP COLUMN availability;

DROP TYPE availability;
