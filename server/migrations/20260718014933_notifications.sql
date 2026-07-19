-- +goose Up
create table if not exists notifications
(
    notification_id uuid        primary key default uuid_generate_v1mc(),
    user_id         uuid        not null references users (user_id) on delete cascade,
    
    type            text        not null,
    data            jsonb       not null,

    read            boolean     not null default false,

    created_at      timestamptz not null default now(),
    updated_at      timestamptz
);

create table if not exists notification_actions
(
    action_id       uuid        primary key default uuid_generate_v1mc(),
    notification_id uuid        not null references notifications (notification_id) on delete cascade,
    
    type            text        not null,
    data            jsonb       not null,

    created_at      timestamptz not null default now(),
    updated_at      timestamptz
);

-- +goose Down
drop table if exists notifications;
