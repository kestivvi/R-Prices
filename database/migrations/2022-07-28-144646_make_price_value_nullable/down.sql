-- This file should undo anything in `up.sql`

ALTER TABLE prices
ALTER COLUMN value SET NOT NULL;
