CREATE TYPE ServerStatus AS ENUM (
    'running',
    'starting',
    'stopped',
    'installing',
    'errored',
    'unknown'
);

CREATE TABLE servers (
    id SERIAL PRIMARY KEY,
    uuid CHAR(36) UNIQUE NOT NULL,
    uuid_short CHAR(8) UNIQUE NOT NULL,
    owner_id INT NOT NULL,
    node_id INT NOT NULL,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    template INT NOT NULL,
    status ServerStatus NOT NULL,
    memory INT NOT NULL,
    storage INT NOT NULL,
    cpu_cores INT NOT NULL,
    port INT NOT NULL,
    command TEXT NOT NULL,
    installed BOOLEAN DEFAULT FALSE,

    CONSTRAINT fk_servers_owner
        FOREIGN KEY (owner_id)
        REFERENCES users (id)
        ON DELETE CASCADE,

    CONSTRAINT fk_servers_node
        FOREIGN KEY (node_id)
        REFERENCES nodes (id)
        ON DELETE CASCADE,

    CONSTRAINT fk_servers_template
        FOREIGN KEY (template)
        REFERENCES templates (id)
        ON DELETE CASCADE
);
