CREATE TABLE tokens (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    value TEXT UNIQUE NOT NULL,

    CONSTRAINT fk_tokens_users
        FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
);