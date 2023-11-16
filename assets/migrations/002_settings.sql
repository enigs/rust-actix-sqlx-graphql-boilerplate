----- CREATE SETTINGS TABLE -----
CREATE TABLE settings (
    id CHARACTER VARYING(32) COLLATE __gl_numeric NOT NULL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE,
    updated_at TIMESTAMP WITH TIME ZONE,
    module CHARACTER VARYING(100) COLLATE __gl_numeric UNIQUE,
    content JSONB NOT NULL
);

----- CREATE INDEXES -----
CREATE INDEX idx_settings_module ON settings USING btree (module);

----- CREATE TRIGGERS -----
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON settings FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();
CREATE TRIGGER set_updated_at_insert BEFORE INSERT ON settings FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_updated_at_update BEFORE UPDATE ON settings FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
