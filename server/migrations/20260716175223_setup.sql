-- +goose Up
create extension if not exists "uuid-ossp";
create extension if not exists "pg_trgm";

-- Some tables have a `created_at` and `updated_at` column which aswell as being useful 
-- information on the frontend such as identifying when a schematic was uplaoded can be 
-- useful for auditing and debugging. `created_at` can be handled by setting a default
-- value for the column as `now()` updated_at would need to be set directly whenever the
-- table is updated or added indivdually as a trigger
--
-- +goose StatementBegin
create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;
-- +goose StatementEnd

-- +goose StatementBegin
create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
    execute format(
        'CREATE TRIGGER set_updated_at
            BEFORE UPDATE
            ON %s
            FOR EACH ROW
            WHEN (OLD is distinct from NEW)
        EXECUTE FUNCTION set_updated_at();', 
        tablename
    );
end;
$$ language plpgsql;
-- +goose StatementEnd

-- This creates a text collation that allows text to be sorted case insensitively useful
-- for unique indexes on usernames and alike allowing us to ensure names are unique whithout
-- being constrained in case
create collation case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);

-- +goose Down
drop function if exists trigger_updated_at(regclass);

drop function if exists set_updated_at();

drop collation if exists case_insensitive;

drop extension if exists "pg_trgm";

drop extension if exists "uuid-ossp";
