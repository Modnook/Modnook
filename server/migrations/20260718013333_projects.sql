-- +goose Up
create type project_role as enum (
    'owner',
    'admin',
    'maintainer'
);

create table if not exists project_types
(
    project_type_id uuid        primary key default uuid_generate_v1mc(),
    
    -- the english name of the project type, e.g. "Mod", "Resource Pack", etc.
    name            text        not null,
    -- the slug of the project type, e.g. "mod", "resource-pack", etc.
    slug            text        not null unique collate case_insensitive,
    description     text,

    created_at      timestamptz not null default now(),
    updated_at      timestamptz
);

create table if not exists project_type_schemas
(
    schema_id       uuid        primary key default uuid_generate_v1mc(),
    project_type_id uuid        not null references project_types (project_type_id) on delete cascade,
    
    schema_version  int         not null,

    schema_json     jsonb       not null,

    created_at      timestamptz not null default now(),
    updated_at      timestamptz,

    unique (project_type_id, schema_version)
);

create table if not exists projects
(
    project_id      uuid        primary key default uuid_generate_v1mc(),
    project_type_id uuid        not null references project_types (project_type_id) on delete cascade,
    
    name            text        not null,
    slug            text        not null unique collate case_insensitive,
    description     text,

    project_icon_id uuid        references assets (asset_id) on delete set null,

    schema_json     jsonb       not null,

    created_at      timestamptz not null default now(),
    updated_at      timestamptz
);

create table if not exists project_memberships
(
    membership_id   uuid         primary key default uuid_generate_v1mc(),
    project_id      uuid         not null references projects (project_id) on delete cascade,
    user_id         uuid         not null references users (user_id) on delete cascade,
    
    role            project_role not null,

    created_at      timestamptz  not null default now(),
    updated_at      timestamptz,

    unique (project_id, user_id)
);

create table if not exists project_invitations
(
    invitation_id   uuid         primary key default uuid_generate_v1mc(),
    project_id      uuid         not null references projects (project_id) on delete cascade,
    inviter_user_id uuid         not null references users (user_id) on delete cascade,
    invited_user_id uuid         not null references users (user_id) on delete cascade,

    role            project_role not null,

    created_at      timestamptz  not null default now(),
    updated_at      timestamptz,

    unique (project_id, invited_user_id)
);

-- +goose Down
drop table if exists project_invitations;
drop table if exists project_memberships;
drop table if exists projects;
drop table if exists project_type_schemas;
drop table if exists project_types; 
