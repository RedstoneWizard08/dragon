CREATE TABLE template_file_configs (
    id SERIAL PRIMARY KEY,
    template INT NOT NULL,
    parser TEXT NOT NULL,
    find TEXT NOT NULL,
    replace TEXT NOT NULL,

    CONSTRAINT fk_template_file_configs_template
        FOREIGN KEY (template)
        REFERENCES templates (id)
        ON DELETE CASCADE
);
