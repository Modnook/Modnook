-- +goose Up
create table if not exists games 
(
    game_id       uuid        primary key default uuid_generate_v1mc(),

    name          text        not null,
    slug          text        not null unique collate case_insensitive,

    description   text,
    icon_id       uuid        references assets (asset_id) on delete set null,
    banner_id     uuid        references assets (asset_id) on delete set null,

    project_count integer     not null default 0,

    created_at    timestamptz not null default now(),
    updated_at    timestamptz
);

-- +goose Down
drop table if exists games;
