-- Add migration script here
ALTER TABLE
    account
ALTER COLUMN
    created_at TYPE timestamptz;


ALTER TABLE
    account
ALTER COLUMN
    updated_at TYPE timestamptz;