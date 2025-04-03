-- Add migration script here
ALTER TABLE contacts
RENAME TO contacts_old;

CREATE TABLE contacts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name TEXT NOT NULL,
    last_name TEXT,
    display_name TEXT,
    email TEXT,
    phone_number TEXT,
    birthday DATE
);

INSERT INTO
    contacts (
        id,
        first_name,
        last_name,
        display_name,
        email,
        phone_number
    )
SELECT
    id,
    first_name,
    last_name,
    display_name,
    email,
    phone_number
FROM
    contacts_old;

ALTER TABLE contacts_metadata
RENAME TO contacts_metadata_old;

CREATE TABLE contacts_metadata (
    contact_id INTEGER NOT NULL,
    starred BOOLEAN NOT NULL,
    is_archived BOOLEAN NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    last_seen_at TEXT,
    next_reminder_at TEXT,
    frequency INTEGER,
    last_reminder_at TEXT,
    FOREIGN KEY (contact_id) REFERENCES contacts (id) ON DELETE CASCADE
);

INSERT INTO
    contacts_metadata
SELECT
    *
FROM
    contacts_metadata_old;

DROP TABLE contacts_metadata_old;

DROP TABLE contacts_old;
