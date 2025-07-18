-- Combine contacts and contacts_metadata tables into a single contacts table
PRAGMA foreign_keys = OFF;

-- Create new combined contacts table
CREATE TABLE contacts_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name TEXT NOT NULL,
    last_name TEXT,
    display_name TEXT,
    email TEXT,
    phone_number TEXT,
    birthday DATE,
    starred BOOLEAN NOT NULL DEFAULT 0,
    is_archived BOOLEAN NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    last_seen_at TEXT,
    next_reminder_at TEXT,
    frequency INTEGER,
    last_reminder_at TEXT
);

-- Copy data from contacts and contacts_metadata tables
INSERT INTO contacts_new (
    id,
    first_name,
    last_name,
    display_name,
    email,
    phone_number,
    birthday,
    starred,
    is_archived,
    created_at,
    updated_at,
    last_seen_at,
    next_reminder_at,
    frequency,
    last_reminder_at
)
SELECT 
    c.id,
    c.first_name,
    c.last_name,
    c.display_name,
    c.email,
    c.phone_number,
    c.birthday,
    COALESCE(cm.starred, 0) as starred,
    COALESCE(cm.is_archived, 0) as is_archived,
    COALESCE(cm.created_at, datetime('now')) as created_at,
    COALESCE(cm.updated_at, datetime('now')) as updated_at,
    cm.last_seen_at,
    cm.next_reminder_at,
    cm.frequency,
    cm.last_reminder_at
FROM contacts c
LEFT JOIN contacts_metadata cm ON c.id = cm.contact_id;

-- Drop old tables
DROP TABLE contacts_metadata;
DROP TABLE contacts;

-- Rename new table to contacts
ALTER TABLE contacts_new RENAME TO contacts;

PRAGMA foreign_keys = ON;