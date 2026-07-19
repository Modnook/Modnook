-- +goose Up
create table if not exists assets
(
    asset_id      uuid        primary key default uuid_generate_v1mc(),
    
    bucket        text        not null,
    object_key    text        not null,

    sha256_hash   text        not null,
    size_bytes    bigint      not null,
    content_type  text        not null,

    created_at    timestamptz not null default now(),
    updated_at    timestamptz,

    unique (sha256_hash, size_bytes)
);

-- +goose Down
drop table if exists assets;
