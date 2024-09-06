-- Add migration script here
CREATE TABLE IF NOT EXISTS contacts
(
id INTEGER PRIMARY KEY AUTOINCREMENT,
first_name TEXT NOT NULL,
last_name TEXT NOT NULL,
display_name TEXT NOT NULL,
email TEXT NOT NULL,
phone_number TEXT NOT NULL
);