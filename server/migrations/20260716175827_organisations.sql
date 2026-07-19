-- +goose Up
create table if not exists organisations
(
    organisation_id uuid        primary key default uuid_generate_v1mc(),
    
    name            text        not null,
    slug            text        not null unique collate case_insensitive,
    description     text,
    logo_url        text,

    created_at      timestamptz not null default now(),
    updated_at      timestamptz
);

create table if not exists organisation_memberships
(
    membership_id   uuid        primary key default uuid_generate_v1mc(),
    organisation_id uuid        not null references organisations (organisation_id) on delete cascade,
    user_id         uuid        not null references users (user_id) on delete cascade,
    
    role            text        not null,

    created_at      timestamptz not null default now(),
    updated_at      timestamptz,

    unique (organisation_id, user_id)
);

-- +goose Down
drop table if exists organisation_memberships;

drop table if exists organisations;

