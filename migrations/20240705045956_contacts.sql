-- Add migration script here
CREATE TABLE IF NOT EXISTS contacts 
(
    id BIGSERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone_number TEXT NOT NULL
);