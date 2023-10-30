ALTER TABLE templates DROP COLUMN group_id;
ALTER TABLE templates DROP CONSTRAINT IF EXISTS fk_templates_group_id;
