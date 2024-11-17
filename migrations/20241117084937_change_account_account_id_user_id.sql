-- Add migration script here

ALTER TABLE
    account
ALTER COLUMN
    account_id TYPE BIGINT;


ALTER TABLE
    account
ALTER COLUMN
    user_id TYPE BIGINT;