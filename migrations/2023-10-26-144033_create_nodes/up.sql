CREATE TABLE nodes (
    id SERIAL PRIMARY KEY,
    uuid CHAR(36) UNIQUE NOT NULL,
    public BOOLEAN NOT NULL DEFAULT FALSE,
    name TEXT NOT NULL,
    ip TEXT NOT NULL,
    scheme TEXT NOT NULL DEFAULT 'https',
    memory INT NOT NULL,
    memory_overallocate BOOLEAN NOT NULL DEFAULT FALSE,
    storage INT NOT NULL,
    storage_overallocate BOOLEAN NOT NULL DEFAULT FALSE,
    daemon_port INT NOT NULL DEFAULT 8090,
    sftp_port INT NOT NULL DEFAULT 2023
);
