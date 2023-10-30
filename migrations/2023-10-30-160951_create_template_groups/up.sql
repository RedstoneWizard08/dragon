CREATE TABLE template_groups (
    id SERIAL PRIMARY KEY,
    uuid CHAR(36) UNIQUE NOT NULL,
    name TEXT UNIQUE NOT NULL,
    description TEXT NOT NULL
);
