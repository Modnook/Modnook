-- +goose Up
create type user_role as enum (
    'user',
    'admin',
    'moderator'
);

create table if not exists users 
(
    user_id        uuid        primary key default uuid_generate_v1mc(),
    
    username       text        not null unique collate case_insensitive,
    displayname    text        not null,
    bio            text,
    avatar_id      uuid        references assets (asset_id) on delete set null,
    role           user_role   not null default 'user'::user_role,
    
    email          text        unique collate case_insensitive,
    email_verified boolean     not null default false,

    password_hash  text,  

    created_at     timestamptz not null default now(),
    updated_at     timestamptz
);

create table if not exists user_identities
(
    identity_id      uuid          primary key default uuid_generate_v1mc(),
    user_id          uuid          not null references users (user_id) on delete cascade,
    
    provider         text          not null,
    provider_user_id text          not null,
    
    metadata         jsonb         not null default '{}'::jsonb,

    created_at       timestamptz   not null default now(),
    updated_at       timestamptz,

    unique (provider, provider_user_id)
);

create index idx_user_identities_user_id on user_identities (user_id);

create table if not exists user_sessions 
(
    session_id uuid        primary key default uuid_generate_v1mc(),
    user_id    uuid        not null references users (user_id) on delete cascade,
    
    ip_address inet,
    user_agent text,
    
    expires_at timestamptz not null,
    revoked_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

create table if not exists user_refresh_tokens 
(
    refresh_token_id uuid        primary key default uuid_generate_v1mc(),
    user_id          uuid        not null references users (user_id) on delete cascade,
    
    token_hash       text        not null unique,
    
    expires_at       timestamptz not null,
    revoked_at       timestamptz,
    created_at       timestamptz not null default now(),
    updated_at       timestamptz
);

-- +goose Down
drop table if exists user_refresh_tokens;

drop table if exists user_sessions;

drop table if exists user_identities;

drop table if exists users;

drop type if exists user_role;
