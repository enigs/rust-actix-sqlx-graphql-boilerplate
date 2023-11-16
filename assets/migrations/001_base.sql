----- Install postgis -----
CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS fuzzystrmatch;

----- CREATE TRIGGERS -----
CREATE FUNCTION __gl_updated_at_now() RETURNS trigger
    LANGUAGE plpgsql
AS $$
BEGIN
    IF row(NEW.*) IS DISTINCT FROM row(OLD.*) THEN
        NEW.updated_at = (now() at time zone 'utc');
RETURN NEW;
ELSE
        RETURN OLD;
END IF;
END;
$$;

CREATE FUNCTION __gl_created_at_now() RETURNS trigger
    LANGUAGE plpgsql
AS $$
BEGIN
    IF row(NEW.*) IS DISTINCT FROM row(OLD.*) THEN
        NEW.created_at = (now() at time zone 'utc');
RETURN NEW;
ELSE
        RETURN OLD;
END IF;
END;
$$;

CREATE FUNCTION __gl_expires_at_30days() RETURNS trigger
    LANGUAGE plpgsql
AS $$
BEGIN
    IF row(NEW.*) IS DISTINCT FROM row(OLD.*) THEN
        NEW.expires_at = (now() + interval '30 days' at time zone 'utc');
RETURN NEW;
ELSE
        RETURN OLD;
END IF;
END;
$$;

----- CREATE NUMERIC COLLATION -----
CREATE COLLATION __gl_numeric (provider = icu, locale = 'en@colNumeric=yes');

----- CREATE SLUGIFY NAME -----
CREATE OR REPLACE FUNCTION __gl_slugify_name()
    RETURNS TRIGGER AS $$
DECLARE
base_slug text;
    generated_slug text;
    final_slug text;
    counter int := 0;
    max_counter int := 0;
    slug_exists boolean;
    target_table text := TG_TABLE_NAME;
BEGIN
    -- Truncate name if it exceeds 100 characters
    base_slug := LEFT(NEW.name, 100);

    -- Generate initial slug
    generated_slug := replace(lower(regexp_replace(base_slug, '[^\w]+', '-', 'g')), ' ', '-');

    -- Remove hyphen at the start or end of the slug
    generated_slug := regexp_replace(generated_slug, '^-+', '');
    generated_slug := regexp_replace(generated_slug, '-+$', '');
    base_slug := generated_slug;

    -- Check if slug already exists
EXECUTE 'SELECT EXISTS(SELECT 1 FROM ' || target_table || ' WHERE slug = $1)' INTO slug_exists USING generated_slug;

-- If slug exists, find the maximum counter value for the given slug
IF slug_exists THEN
        -- Increment the counter by 1
        counter := max_counter + 1;
END IF;

    -- Append the counter to the slug
    generated_slug := base_slug || '-' || counter;

    -- Check if the new generated slug already exists
EXECUTE 'SELECT EXISTS(SELECT 1 FROM ' || target_table || ' WHERE slug = $1)' INTO slug_exists USING generated_slug;

-- Increment the counter until a unique slug is found
WHILE slug_exists LOOP
            -- Find the maximum counter value for the given slug
            EXECUTE 'SELECT MAX(CAST(SUBSTRING(slug, LENGTH($1) + 2) AS INTEGER))
                  FROM ' || target_table || '
                  WHERE slug ~ CONCAT($1, ''-[0-9]+$'')' INTO counter USING base_slug;

            counter := counter + 1;
            generated_slug := base_slug || '-' || counter;
EXECUTE 'SELECT EXISTS(SELECT 1 FROM ' || target_table || ' WHERE slug = $1)' INTO slug_exists USING generated_slug;
END LOOP;

    IF counter > 0 THEN
        final_slug := base_slug || '-' || counter;
ELSE
        final_slug := base_slug;
END IF;

    -- Assign the generated slug to the NEW record
    NEW.slug := final_slug;

RETURN NEW;
END;
$$ LANGUAGE plpgsql;

----- CREATE SLUGIFY ACTOR -----
CREATE OR REPLACE FUNCTION __gl_slugify_actor()
    RETURNS TRIGGER AS $$
DECLARE
base_slug text;
    generated_slug text;
    final_slug text;
    counter int := 0;
    max_counter int := 0;
    slug_exists boolean;
    target_table text := TG_TABLE_NAME;
BEGIN
    -- Truncate name if it exceeds 100 characters
    base_slug := LEFT(NEW.first_name || '-' || NEW.last_name, 100);

    -- Generate initial slug
    generated_slug := replace(lower(regexp_replace(base_slug, '[^\w]+', '-', 'g')), ' ', '-');

    -- Remove hyphen at the start or end of the slug
    generated_slug := regexp_replace(generated_slug, '^-+', '');
    generated_slug := regexp_replace(generated_slug, '-+$', '');
    base_slug := generated_slug;

    -- Check if slug already exists
EXECUTE 'SELECT EXISTS(SELECT 1 FROM ' || target_table || ' WHERE slug = $1)' INTO slug_exists USING generated_slug;

-- If slug exists, find the maximum counter value for the given slug
IF slug_exists THEN
        -- Increment the counter by 1
        counter := max_counter + 1;
END IF;

    -- Append the counter to the slug
    generated_slug := base_slug || '-' || counter;

    -- Check if the new generated slug already exists
EXECUTE 'SELECT EXISTS(SELECT 1 FROM ' || target_table || ' WHERE slug = $1)' INTO slug_exists USING generated_slug;

-- Increment the counter until a unique slug is found
WHILE slug_exists LOOP
            -- Find the maximum counter value for the given slug
            EXECUTE 'SELECT MAX(CAST(SUBSTRING(slug, LENGTH($1) + 2) AS INTEGER))
                  FROM ' || target_table || '
                  WHERE slug ~ CONCAT($1, ''-[0-9]+$'')' INTO counter USING base_slug;

            counter := counter + 1;
            generated_slug := base_slug || '-' || counter;
EXECUTE 'SELECT EXISTS(SELECT 1 FROM ' || target_table || ' WHERE slug = $1)' INTO slug_exists USING generated_slug;
END LOOP;

    IF counter > 0 THEN
        final_slug := base_slug || '-' || counter;
ELSE
        final_slug := base_slug;
END IF;

    -- Assign the generated slug to the NEW record
    NEW.slug := final_slug;

RETURN NEW;
END;
$$ LANGUAGE plpgsql;