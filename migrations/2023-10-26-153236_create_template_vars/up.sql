CREATE TYPE VarType AS ENUM (
    'text',
    'number'
);

CREATE TABLE template_vars (
    id SERIAL PRIMARY KEY,
    template INT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    env_var TEXT,
    default_value TEXT,
    editable BOOLEAN NOT NULL DEFAULT FALSE,
    rules TEXT NOT NULL,
    type VarType NOT NULL,

    CONSTRAINT fk_template_vars_template
        FOREIGN KEY (template)
        REFERENCES templates (id)
        ON DELETE CASCADE
);
