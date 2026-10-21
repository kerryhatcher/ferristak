-- 0001_init.sql — initial Team Awareness Kit schema.

create extension if not exists pgcrypto;

-- ---------------------------------------------------------------------------
-- Operators (users)
-- ---------------------------------------------------------------------------
create table if not exists users (
    id            uuid primary key default gen_random_uuid(),
    email         text        not null unique,
    callsign      text        not null,
    password_hash text        not null,
    role          text        not null default 'operator'
                              check (role in ('operator', 'admin')),
    created_at    timestamptz not null default now(),
    updated_at    timestamptz not null default now()
);

-- ---------------------------------------------------------------------------
-- Position / status check-ins
-- ---------------------------------------------------------------------------
create table if not exists checkins (
    id         uuid primary key default gen_random_uuid(),
    user_id    uuid not null references users (id) on delete cascade,
    latitude   double precision not null check (latitude between -90 and 90),
    longitude  double precision not null check (longitude between -180 and 180),
    status     text not null default 'ok'
                    check (status in ('ok', 'attention', 'help', 'offline')),
    note       text,
    created_at timestamptz not null default now()
);

create index if not exists checkins_user_id_created_at_idx
    on checkins (user_id, created_at desc);

create index if not exists checkins_created_at_idx
    on checkins (created_at desc);

-- ---------------------------------------------------------------------------
-- updated_at maintenance
-- ---------------------------------------------------------------------------
create or replace function set_updated_at()
returns trigger as $$
begin
    new.updated_at = now();
    return new;
end;
$$ language plpgsql;

drop trigger if exists users_set_updated_at on users;
create trigger users_set_updated_at
    before update on users
    for each row execute function set_updated_at();
