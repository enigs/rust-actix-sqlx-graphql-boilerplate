-------------------------------
----- CREATE IMPORT TABLE -----
-------------------------------
CREATE TABLE IF NOT EXISTS import (
    id CHARACTER VARYING(32) COLLATE __gl_numeric NOT NULL PRIMARY KEY,
    cursor BIGSERIAL UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE,
    updated_at TIMESTAMP WITH TIME ZONE,
    created_by_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    filename CHARACTER VARYING(75) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    label CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    sheet_name CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    module CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    status CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL,
    created_rows BIGINT DEFAULT 0,
    updated_rows BIGINT DEFAULT 0,
    deleted_rows BIGINT DEFAULT 0,
    mapped_fields BIGINT DEFAULT 0,
    unmapped_fields BIGINT DEFAULT 0,
    total_cells BIGINT DEFAULT 0,
    total_non_empty_cells BIGINT DEFAULT 0,
    total_rows BIGINT DEFAULT 0,
    total_cols BIGINT DEFAULT 0,
    current_row BIGINT DEFAULT 0,
    failed_rows BIGINT DEFAULT 0,
    ignored_rows BIGINT DEFAULT 0,
    rejected_rows BIGINT DEFAULT 0,
    mapping JSONB DEFAULT NULL
);

----- CREATE IMPORT INDEXES -----
CREATE INDEX idx_import_created_by_id ON import USING btree (created_by_id);
CREATE INDEX idx_import_created_at ON import USING btree (created_at);
CREATE INDEX idx_import_updated_at ON import USING btree (updated_at);
CREATE INDEX idx_import_filename ON import USING btree (filename);
CREATE INDEX idx_import_module ON import USING btree (module);
CREATE INDEX idx_import_status ON import USING btree (status);

----- CREATE IMPORT TRIGGERS -----
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON import FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();
CREATE TRIGGER set_updated_at_insert BEFORE INSERT ON import FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_updated_at_update BEFORE UPDATE ON import FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();

-------------------------------
------ CREATE FILE TABLE ------
-------------------------------
CREATE TABLE IF NOT EXISTS file (
    id CHARACTER VARYING(32) COLLATE __gl_numeric NOT NULL PRIMARY KEY,
    cursor BIGSERIAL UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    expires_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    created_by_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    message_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    filename CHARACTER VARYING(75) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    video_url TEXT COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    video_source CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    video_thumbnail_s_m CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    video_thumbnail_m_d CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    video_thumbnail_l_g CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    video_thumbnail_x_l CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    thumbnail_s_m CHARACTER VARYING(75) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    thumbnail_m_d CHARACTER VARYING(75) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    thumbnail_l_g CHARACTER VARYING(75) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    thumbnail_x_l CHARACTER VARYING(75) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    landscape_s_m CHARACTER VARYING(75) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    landscape_m_d CHARACTER VARYING(75) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    landscape_l_g CHARACTER VARYING(75) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    landscape_x_l CHARACTER VARYING(75) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    landscape_x_x_l CHARACTER VARYING(75) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    landscape_x_x_x_l CHARACTER VARYING(75) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    module CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    label TEXT COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    extension CHARACTER VARYING(5) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    description TEXT COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    status CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL,
    mime_type CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    file_size CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    file_type CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    height CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    width CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    is_attached BOOLEAN DEFAULT NULL,
    is_thumbnail BOOLEAN DEFAULT NULL
);

------ CREATE FILE INDEXES ------
CREATE INDEX idx_file_filename ON file USING btree (filename);
CREATE INDEX idx_file_description ON file USING btree (description);
CREATE INDEX idx_file_status ON file USING btree (status);
CREATE INDEX idx_file_mime_type ON file USING btree (mime_type);
CREATE INDEX idx_file_is_attached ON file USING btree (is_attached);
CREATE INDEX idx_file_expires_at ON file USING btree (expires_at);
CREATE INDEX idx_file_created_by_id ON file USING btree (created_by_id);
CREATE INDEX idx_file_message_id ON file USING btree (message_id);
CREATE INDEX idx_file_created_at ON file USING btree (created_at);
CREATE INDEX idx_file_updated_at ON file USING btree (updated_at);

----- CREATE FILE TRIGGERS -----
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON file FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();
CREATE TRIGGER set_updated_at_insert BEFORE INSERT ON file FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_updated_at_update BEFORE UPDATE ON file FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_expires_at_insert BEFORE INSERT ON file FOR EACH ROW EXECUTE FUNCTION __gl_expires_at_30days();

-------------------------------
---- CREATE CATEGORY TABLE ----
-------------------------------
CREATE TABLE category (
    id CHARACTER VARYING(32) COLLATE __gl_numeric NOT NULL PRIMARY KEY,
    cursor BIGSERIAL UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    created_by_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    import_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    import_index INTEGER DEFAULT NULL,
    parent_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    image_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    name CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    slug CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING UNIQUE,
    status CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL,
    is_admin_featured BOOLEAN DEFAULT NULL,
    is_platform_featured BOOLEAN DEFAULT NULL
);

---- CREATE CATEGORY INDEXES ----
CREATE INDEX idx_category_name ON category USING btree (name);
CREATE INDEX idx_category_slug ON category USING btree (slug);
CREATE INDEX idx_category_status ON category USING btree (status);
CREATE INDEX idx_category_parent_id ON category USING btree (parent_id);
CREATE INDEX idx_category_image_id ON category USING btree (image_id);
CREATE INDEX idx_category_import_id ON category USING btree (import_id);
CREATE INDEX idx_category_created_by_id ON category USING btree (created_by_id);
CREATE INDEX idx_category_created_at ON category USING btree (created_at);
CREATE INDEX idx_category_updated_at ON category USING btree (updated_at);
CREATE INDEX idx_category_is_admin_featured ON category USING btree (is_admin_featured);
CREATE INDEX idx_category_is_platform_featured ON category USING btree (is_platform_featured);

---- CREATE CATEGORY TRIGGERS ----
CREATE TRIGGER slugify_category_name BEFORE INSERT ON category FOR EACH ROW EXECUTE FUNCTION __gl_slugify_name();
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON category FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();
CREATE TRIGGER set_updated_at_insert BEFORE INSERT ON category FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_updated_at_update BEFORE UPDATE ON category FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();

---- CREATE CATEGORY CONSTRAINTS ----
ALTER TABLE ONLY category
    ADD CONSTRAINT fk_category_file FOREIGN KEY (image_id) REFERENCES file(id) ON DELETE SET NULL;

-------------------------------
---- CREATE COMPANY TABLE ----
-------------------------------
CREATE TABLE company (
    id CHARACTER VARYING(32) COLLATE __gl_numeric NOT NULL PRIMARY KEY,
    cursor BIGSERIAL UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    created_by_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    import_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    import_index INTEGER DEFAULT NULL,
    banner_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    logo_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    org_admin_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    name CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    slug CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING UNIQUE,
    role CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL,
    status CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL,
    business_description text COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    street CHARACTER VARYING(500) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    city CHARACTER VARYING(500) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    state CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    zip CHARACTER VARYING(15) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    country CHARACTER VARYING(100) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    coordinates GEOMETRY(POINT, 4326) DEFAULT NULL,
    website CHARACTER VARYING(500) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    facebook CHARACTER VARYING(500) COLLATE __gl_numeric DEFAULT NULL,
    linkedin CHARACTER VARYING(500) COLLATE __gl_numeric DEFAULT NULL,
    landline TEXT[] COLLATE __gl_numeric DEFAULT NULL,
    mobile TEXT[] COLLATE __gl_numeric DEFAULT NULL,
    is_admin_featured BOOLEAN DEFAULT NULL,
    is_platform_featured BOOLEAN DEFAULT NULL
);

---- CREATE COMPANY INDEXES ----
CREATE INDEX idx_company_name ON company USING btree (name);
CREATE INDEX idx_company_slug ON company USING btree (slug);
CREATE INDEX idx_company_role ON company USING btree (role);
CREATE INDEX idx_company_status ON company USING btree (status);
CREATE INDEX idx_company_street ON company USING btree (street);
CREATE INDEX idx_company_city ON company USING btree (city);
CREATE INDEX idx_company_state ON company USING btree (state);
CREATE INDEX idx_company_zip ON company USING btree (zip);
CREATE INDEX idx_company_country ON company USING btree (country);
CREATE INDEX idx_company_coordinates ON company USING btree (coordinates);
CREATE INDEX idx_company_banner_id ON company USING btree (banner_id);
CREATE INDEX idx_company_logo_id ON company USING btree (logo_id);
CREATE INDEX idx_company_import_id ON company USING btree (import_id);
CREATE INDEX idx_company_created_by_id ON company USING btree (created_by_id);
CREATE INDEX idx_company_org_admin_id ON company USING btree (org_admin_id);
CREATE INDEX idx_company_is_admin_featured ON company USING btree (is_admin_featured);
CREATE INDEX idx_company_is_platform_featured ON company USING btree (is_platform_featured);
CREATE INDEX idx_company_created_at ON company USING btree (created_at);
CREATE INDEX idx_company_updated_at ON company USING btree (updated_at);

---- CREATE COMPANY TRIGGERS ----
CREATE TRIGGER slugify_company_name BEFORE INSERT ON company FOR EACH ROW EXECUTE FUNCTION __gl_slugify_name();
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON company FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();
CREATE TRIGGER set_updated_at_insert BEFORE INSERT ON company FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_updated_at_update BEFORE UPDATE ON company FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();

---- CREATE COMPANY CONSTRAINTS ----
ALTER TABLE ONLY company
    ADD CONSTRAINT fk_company_banner FOREIGN KEY (banner_id) REFERENCES file(id) ON DELETE SET NULL;
ALTER TABLE ONLY company
    ADD CONSTRAINT fk_company_logo FOREIGN KEY (logo_id) REFERENCES file(id) ON DELETE SET NULL;

-------------------------------
---- CREATE ADDRESS TABLE ----
-------------------------------
CREATE TABLE address (
    id CHARACTER VARYING(32) COLLATE __gl_numeric NOT NULL PRIMARY KEY,
    cursor BIGSERIAL UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    created_by_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    import_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    import_index INTEGER DEFAULT NULL,
    company_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    name CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    description text COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    street CHARACTER VARYING(500) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    city CHARACTER VARYING(500) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    state CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    zip CHARACTER VARYING(15) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    country CHARACTER VARYING(100) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    coordinates GEOMETRY(POINT, 4326) DEFAULT NULL,
    is_default BOOLEAN DEFAULT NULL
);

---- CREATE ADDRESS INDEXES ----
CREATE INDEX idx_address_company_id ON address USING btree (company_id);
CREATE INDEX idx_address_name ON address USING btree (name);
CREATE INDEX idx_address_street ON address USING btree (street);
CREATE INDEX idx_address_city ON address USING btree (city);
CREATE INDEX idx_address_state ON address USING btree (state);
CREATE INDEX idx_address_zip ON address USING btree (zip);
CREATE INDEX idx_address_country ON address USING btree (country);
CREATE INDEX idx_address_coordinates ON address USING btree (coordinates);
CREATE INDEX idx_address_is_default ON address USING btree (is_default);
CREATE INDEX idx_address_created_at ON address USING btree (created_at);
CREATE INDEX idx_address_updated_at ON address USING btree (updated_at);

---- CREATE ADDRESS TRIGGERS ----
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON address FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();
CREATE TRIGGER set_updated_at_insert BEFORE INSERT ON address FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_updated_at_update BEFORE UPDATE ON address FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();

---- CREATE ADDRESS CONSTRAINTS ----
ALTER TABLE ONLY address
    ADD CONSTRAINT fk_address_company FOREIGN KEY (company_id) REFERENCES company(id) ON DELETE SET NULL;

-------------------------------
----- CREATE ACTOR TABLE -----
-------------------------------
CREATE TABLE actor (
    id CHARACTER VARYING(32) COLLATE __gl_numeric NOT NULL PRIMARY KEY,
    cursor BIGSERIAL UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    created_by_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    import_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    import_index INTEGER DEFAULT NULL,
    company_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    image_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    registration_info JSONB DEFAULT NULL,
    account_actions JSONB DEFAULT NULL,
    account_verification JSONB DEFAULT NULL,
    account_reset_password JSONB DEFAULT NULL,
    account_sign_in JSONB DEFAULT NULL,
    wizard JSONB DEFAULT NULL,
    account_type CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    org_ownership_type CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    email CHARACTER VARYING(300) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING UNIQUE,
    alternate_email CHARACTER VARYING(300) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    password jsonb DEFAULT NULL,
    first_name CHARACTER VARYING(300) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    last_name CHARACTER VARYING(300) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    slug CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING UNIQUE,
    role CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    status CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL,
    street CHARACTER VARYING(500) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    city CHARACTER VARYING(500) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    state CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    zip CHARACTER VARYING(15) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    country CHARACTER VARYING(100) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    coordinates GEOMETRY(POINT, 4326) DEFAULT NULL,
    website CHARACTER VARYING(500) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    facebook CHARACTER VARYING(500) COLLATE __gl_numeric DEFAULT NULL,
    linkedin CHARACTER VARYING(500) COLLATE __gl_numeric DEFAULT NULL,
    landline TEXT[] COLLATE __gl_numeric DEFAULT NULL,
    mobile TEXT[] COLLATE __gl_numeric DEFAULT NULL,
    is_subscribed_to_newsletter boolean DEFAULT false
);

---- CREATE ACTOR INDEXES ----
CREATE INDEX idx_actor_first_name ON actor USING btree (first_name);
CREATE INDEX idx_actor_last_name ON actor USING btree (last_name);
CREATE INDEX idx_actor_email ON actor USING btree (email);
CREATE INDEX idx_actor_alternate_email ON actor USING btree (alternate_email);
CREATE INDEX idx_actor_slug ON actor USING btree (slug);
CREATE INDEX idx_actor_role ON actor USING btree (role);
CREATE INDEX idx_actor_status ON actor USING btree (status);
CREATE INDEX idx_actor_street ON actor USING btree (street);
CREATE INDEX idx_actor_city ON actor USING btree (city);
CREATE INDEX idx_actor_state ON actor USING btree (state);
CREATE INDEX idx_actor_zip ON actor USING btree (zip);
CREATE INDEX idx_actor_country ON actor USING btree (country);
CREATE INDEX idx_actor_coordinates ON actor USING btree (coordinates);
CREATE INDEX idx_actor_image_id ON actor USING btree (image_id);
CREATE INDEX idx_actor_import_id ON actor USING btree (import_id);
CREATE INDEX idx_actor_created_by_id ON actor USING btree (created_by_id);
CREATE INDEX idx_actor_created_at ON actor USING btree (created_at);
CREATE INDEX idx_actor_updated_at ON actor USING btree (updated_at);

---- CREATE ACTOR TRIGGERS ----
CREATE TRIGGER slugify_actor_name BEFORE INSERT ON actor FOR EACH ROW EXECUTE FUNCTION __gl_slugify_actor();
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON actor FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();
CREATE TRIGGER set_updated_at_insert BEFORE INSERT ON actor FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_updated_at_update BEFORE UPDATE ON actor FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();

---- CREATE ACTOR CONSTRAINTS ----
ALTER TABLE ONLY actor
    ADD CONSTRAINT fk_actor_company FOREIGN KEY (company_id) REFERENCES company(id) ON DELETE SET NULL;
ALTER TABLE ONLY actor
    ADD CONSTRAINT fk_actor_file FOREIGN KEY (image_id) REFERENCES file(id) ON DELETE SET NULL;
ALTER TABLE ONLY company
    ADD CONSTRAINT fk_company_actor FOREIGN KEY (org_admin_id) REFERENCES actor(id) ON DELETE SET NULL;

-------------------------------
---- CREATE SESSION TABLE ----
-------------------------------
CREATE TABLE session (
    id CHARACTER VARYING(32) COLLATE __gl_numeric NOT NULL PRIMARY KEY,
    cursor BIGSERIAL UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    expires_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    actor_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    product_name CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    product_major CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    product_minor CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    product_patch CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    os_name CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    os_major CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    os_minor CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    os_patch CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    os_patch_minor CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    device_name CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    device_brand CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    device_model CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    cpu_architecture CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    engine_name CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    engine_major CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    engine_minor CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    engine_patch CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    ip CHARACTER VARYING(260) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    data jsonb DEFAULT NULL
);

---- CREATE SESSION INDEXES ----
CREATE INDEX idx_session_actor_id ON session USING btree (actor_id);
CREATE INDEX idx_session_ip ON session USING btree (ip);

---- CREATE SESSION TRIGGERS ----
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON session FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();
CREATE TRIGGER set_updated_at_insert BEFORE INSERT ON session FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_updated_at_update BEFORE UPDATE ON session FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();

---- CREATE SESSION CONSTRAINTS ----
ALTER TABLE ONLY session
    ADD CONSTRAINT fk_session_actor FOREIGN KEY (actor_id) REFERENCES actor(id) ON DELETE SET NULL;

-------------------------------
--- CREATE CHAT ROOM TABLE ---
-------------------------------
CREATE TABLE chat_room (
    id CHARACTER VARYING(32) COLLATE __gl_numeric NOT NULL PRIMARY KEY,
    cursor BIGSERIAL UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    actor_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    company_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    product_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    status CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL
);

---- CREATE CHAT ROOM INDEXES ----
CREATE INDEX idx_chat_room_actor_id ON chat_room USING btree (actor_id);
CREATE INDEX idx_chat_room_company_id ON chat_room USING btree (company_id);
CREATE INDEX idx_chat_room_product_id ON chat_room USING btree (product_id);
CREATE INDEX idx_chat_room_status ON chat_room USING btree (status);
CREATE INDEX idx_chat_room_created_at ON chat_room USING btree (created_at);
CREATE INDEX idx_chat_room_updated_at ON chat_room USING btree (updated_at);

---- CREATE CHAT ROOM TRIGGERS ----
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON chat_room FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();
CREATE TRIGGER set_updated_at_insert BEFORE INSERT ON chat_room FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_updated_at_update BEFORE UPDATE ON chat_room FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();

---- CREATE CHAT ROOM CONSTRAINTS ----
ALTER TABLE ONLY chat_room
    ADD CONSTRAINT fk_chat_room_actor FOREIGN KEY (actor_id) REFERENCES actor(id) ON DELETE SET NULL;
ALTER TABLE ONLY chat_room
    ADD CONSTRAINT fk_chat_room_company FOREIGN KEY (company_id) REFERENCES company(id) ON DELETE SET NULL;

-------------------------------
----- CREATE TICKET TABLE -----
-------------------------------
CREATE TABLE ticket (
    id BIGSERIAL PRIMARY KEY,
    cursor BIGSERIAL UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    actor_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    company_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    product_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    status CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL
);

---- CREATE TICKET INDEXES ----
CREATE INDEX idx_ticket_actor_id ON ticket USING btree (actor_id);
CREATE INDEX idx_ticket_company_id ON ticket USING btree (company_id);
CREATE INDEX idx_ticket_product_id ON ticket USING btree (product_id);
CREATE INDEX idx_ticket_status ON ticket USING btree (status);
CREATE INDEX idx_ticket_created_at ON ticket USING btree (created_at);
CREATE INDEX idx_ticket_updated_at ON ticket USING btree (updated_at);

---- CREATE TICKET TRIGGERS ----
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON ticket FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();
CREATE TRIGGER set_updated_at_insert BEFORE INSERT ON ticket FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_updated_at_update BEFORE UPDATE ON ticket FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();

---- CREATE TICKET CONSTRAINTS ----
ALTER TABLE ONLY ticket
    ADD CONSTRAINT fk_ticket_actor FOREIGN KEY (actor_id) REFERENCES actor(id) ON DELETE SET NULL;
ALTER TABLE ONLY ticket
    ADD CONSTRAINT fk_ticket_company FOREIGN KEY (company_id) REFERENCES company(id) ON DELETE SET NULL;

-------------------------------
---- CREATE PIPELINE TABLE ----
-------------------------------
CREATE TABLE chat_pipeline (
    id CHARACTER VARYING(32) COLLATE __gl_numeric NOT NULL PRIMARY KEY,
    cursor BIGSERIAL UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    included_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    removed_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    unseen BIGINT DEFAULT NULL,
    chat_room_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    actor_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    status CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL
);

---- CREATE CHAT PIPELINE INDEXES ----
CREATE INDEX idx_chat_pipeline_chat_room_id ON chat_pipeline USING btree (chat_room_id);
CREATE INDEX idx_chat_pipeline_actor_id ON chat_pipeline USING btree (actor_id);
CREATE INDEX idx_chat_pipeline_status ON chat_pipeline USING btree (status);
CREATE INDEX idx_chat_pipeline_created_at ON chat_pipeline USING btree (created_at);
CREATE INDEX idx_chat_pipeline_updated_at ON chat_pipeline USING btree (updated_at);
CREATE INDEX idx_chat_pipeline_included_at ON chat_pipeline USING btree (included_at);
CREATE INDEX idx_chat_pipeline_removed_at ON chat_pipeline USING btree (removed_at);
CREATE INDEX idx_chat_pipeline_unseen ON chat_pipeline USING btree (unseen);

---- CREATE CHAT PIPELINE TRIGGERS ----
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON chat_pipeline FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();
CREATE TRIGGER set_updated_at_insert BEFORE INSERT ON chat_pipeline FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_updated_at_update BEFORE UPDATE ON chat_pipeline FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();

---- CREATE CHAT PIPELINE CONSTRAINTS ----
ALTER TABLE ONLY chat_pipeline
    ADD CONSTRAINT fk_chat_pipeline_actor FOREIGN KEY (actor_id) REFERENCES actor(id) ON DELETE SET NULL;
ALTER TABLE ONLY chat_pipeline
    ADD CONSTRAINT fk_chat_pipeline_chat_room FOREIGN KEY (chat_room_id) REFERENCES chat_room(id) ON DELETE SET NULL;

-------------------------------
---- CREATE PIPELINE TABLE ----
-------------------------------
CREATE TABLE ticket_pipeline (
    id CHARACTER VARYING(32) COLLATE __gl_numeric NOT NULL PRIMARY KEY,
    cursor BIGSERIAL UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    included_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    removed_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    unseen BIGINT DEFAULT NULL,
    ticket_id BIGINT DEFAULT NULL,
    actor_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    status CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL
);

---- CREATE TICKET PIPELINE INDEXES ----
CREATE INDEX idx_ticket_pipeline_ticket_id ON ticket_pipeline USING btree (ticket_id);
CREATE INDEX idx_ticket_pipeline_actor_id ON ticket_pipeline USING btree (actor_id);
CREATE INDEX idx_ticket_pipeline_status ON ticket_pipeline USING btree (status);
CREATE INDEX idx_ticket_pipeline_created_at ON ticket_pipeline USING btree (created_at);
CREATE INDEX idx_ticket_pipeline_updated_at ON ticket_pipeline USING btree (updated_at);
CREATE INDEX idx_ticket_pipeline_included_at ON ticket_pipeline USING btree (included_at);
CREATE INDEX idx_ticket_pipeline_removed_at ON ticket_pipeline USING btree (removed_at);
CREATE INDEX idx_ticket_pipeline_unseen ON ticket_pipeline USING btree (unseen);

---- CREATE TICKET PIPELINE TRIGGERS ----
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON ticket_pipeline FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();
CREATE TRIGGER set_updated_at_insert BEFORE INSERT ON ticket_pipeline FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_updated_at_update BEFORE UPDATE ON ticket_pipeline FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();

---- CREATE TICKET PIPELINE CONSTRAINTS ----
ALTER TABLE ONLY ticket_pipeline
    ADD CONSTRAINT fk_ticket_pipeline_actor FOREIGN KEY (actor_id) REFERENCES actor(id) ON DELETE SET NULL;
ALTER TABLE ONLY ticket_pipeline
    ADD CONSTRAINT fk_ticket_pipeline_ticket FOREIGN KEY (ticket_id) REFERENCES ticket(id) ON DELETE SET NULL;

-------------------------------
---- CREATE MESSAGE TABLE ----
-------------------------------
CREATE TABLE message (
    id CHARACTER VARYING(32) COLLATE __gl_numeric NOT NULL PRIMARY KEY,
    cursor BIGSERIAL UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    removed_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    chat_room_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    ticket_id BIGINT DEFAULT NULL,
    actor_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    file_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    content TEXT COLLATE __gl_numeric DEFAULT NULL,
    message_type CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL,
    status CHARACTER VARYING(50) COLLATE __gl_numeric DEFAULT NULL
);

---- CREATE MESSAGE INDEXES ----
CREATE INDEX idx_message_chat_room_id ON message USING btree (chat_room_id);
CREATE INDEX idx_message_ticket_id ON message USING btree (ticket_id);
CREATE INDEX idx_message_actor_id ON message USING btree (actor_id);
CREATE INDEX idx_message_file_id ON message USING btree (file_id);
CREATE INDEX idx_message_status ON message USING btree (status);
CREATE INDEX idx_message_message_type ON message USING btree (message_type);
CREATE INDEX idx_message_created_at ON message USING btree (created_at);
CREATE INDEX idx_message_updated_at ON message USING btree (updated_at);
CREATE INDEX idx_message_removed_at ON message USING btree (removed_at);

---- CREATE MESSAGE TRIGGERS ----
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON message FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();
CREATE TRIGGER set_updated_at_insert BEFORE INSERT ON message FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();
CREATE TRIGGER set_updated_at_update BEFORE UPDATE ON message FOR EACH ROW EXECUTE FUNCTION __gl_updated_at_now();

---- CREATE MESSAGE CONSTRAINTS ----
ALTER TABLE ONLY message
    ADD CONSTRAINT fk_message_actor FOREIGN KEY (actor_id) REFERENCES actor(id) ON DELETE SET NULL;
ALTER TABLE ONLY message
    ADD CONSTRAINT fk_message_chat_room FOREIGN KEY (chat_room_id) REFERENCES chat_room(id) ON DELETE SET NULL;
ALTER TABLE ONLY message
    ADD CONSTRAINT fk_message_ticket FOREIGN KEY (ticket_id) REFERENCES ticket(id) ON DELETE SET NULL;
ALTER TABLE ONLY message
    ADD CONSTRAINT fk_message_file FOREIGN KEY (file_id) REFERENCES file(id) ON DELETE SET NULL;

-----------------------------------
---- CREATE MESSAGE VIEW TABLE ----
-----------------------------------
CREATE TABLE message_view (
    id CHARACTER VARYING(32) COLLATE __gl_numeric NOT NULL PRIMARY KEY,
    cursor BIGSERIAL UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    message_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING,
    actor_id CHARACTER VARYING(32) COLLATE __gl_numeric DEFAULT NULL::CHARACTER VARYING
);

---- CREATE MESSAGE VIEW INDEXES ----
CREATE INDEX idx_message_view_message_id ON message_view USING btree (message_id);
CREATE INDEX idx_message_view_actor_id ON message_view USING btree (actor_id);
CREATE INDEX idx_message_view_created_at ON message_view USING btree (created_at);

---- CREATE MESSAGE VIEW TRIGGERS ----
CREATE TRIGGER set_created_at_insert BEFORE INSERT ON message_view FOR EACH ROW EXECUTE FUNCTION __gl_created_at_now();

---- CREATE MESSAGE VIEW CONSTRAINTS ----
ALTER TABLE ONLY message_view
    ADD CONSTRAINT fk_message_view_actor FOREIGN KEY (actor_id) REFERENCES actor(id) ON DELETE SET NULL;
ALTER TABLE ONLY message_view
    ADD CONSTRAINT fk_message_view_message FOREIGN KEY (message_id) REFERENCES message(id) ON DELETE SET NULL;