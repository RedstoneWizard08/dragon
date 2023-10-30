CREATE TABLE templates (
    id SERIAL PRIMARY KEY,
    uuid CHAR(36) UNIQUE NOT NULL,
    name TEXT UNIQUE NOT NULL,
    author TEXT NOT NULL,
    description TEXT NOT NULL,
    docker_images TEXT[] NOT NULL,
    install_script TEXT NOT NULL,
    install_image TEXT NOT NULL,
    install_entrypoint TEXT NOT NULL DEFAULT 'sh',
    startup_command TEXT NOT NULL,
    stop_command TEXT NOT NULL DEFAULT 'stop',
    startup_message TEXT,
    log_file TEXT,
    custom_log_file BOOLEAN NOT NULL DEFAULT FALSE
);
