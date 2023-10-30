ALTER TABLE templates ADD group_id INT NOT NULL;
ALTER TABLE templates ADD CONSTRAINT fk_templates_group_id FOREIGN KEY (group_id) REFERENCES template_groups (id);
