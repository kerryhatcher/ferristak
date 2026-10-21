-- 0002_cot_events.sql
--
-- Replace the placeholder `checkins` table with Cursor-on-Target (CoT) events.
--
-- The column set and constraints mirror the public CoT schemas:
--   * CoT Base-Event Schema 2.0 (event, point, detail)   [MITRE / MIL-STD-6090]
--   * xsd/details/*.xsd  (contact, __group, status, track, takv, precisionlocation, remarks)
--
-- The `type`, `how` and `qos` checks reproduce the XML Schema `xs:pattern`s so
-- that invalid CoT never reaches the table.

drop table if exists checkins;

create table if not exists cot_events (
    id              uuid primary key default gen_random_uuid(),
    owner_id        uuid not null references users (id) on delete cascade,

    -- ---- <event> attributes -------------------------------------------------
    cot_version     double precision not null default 2.0 check (cot_version >= 2),
    event_type      text not null check (event_type ~ '^\w+(-\w+)*(;[^;]*)?$'),
    access          text not null default 'Undefined' check (length(access) > 0),
    caveat          text,
    releasable_to   text,
    qos             text check (qos is null or qos ~ '^\d-\w-\w$'),
    opex            text,
    uid             text not null check (length(uid) > 0),
    send_time       timestamptz not null,   -- <event time="…">
    start_time      timestamptz not null,   -- <event start="…">
    stale_time      timestamptz not null,   -- <event stale="…">
    how             text not null check (how ~ '^\w(-\w+)*$'),

    -- ---- <point> ------------------------------------------------------------
    lat             double precision not null check (lat between -90 and 90),
    lon             double precision not null check (lon between -180 and 180),
    hae             double precision not null default 999999,
    ce              double precision not null default 999999,
    le              double precision not null default 999999,

    -- ---- <detail> strongly-typed sub-elements ------------------------------
    contact_callsign text,
    contact_endpoint text,
    group_name       text,
    group_role       text,
    battery          integer check (battery between 0 and 100),
    readiness        boolean,
    track_speed      double precision,
    track_course     double precision,
    track_slope      double precision,
    takv_device      text,
    takv_platform    text,
    takv_os          text,
    takv_version     text,
    geopointsrc      text,
    altsrc           text,
    remarks          text,

    -- Any <detail> content not covered above, preserved verbatim (cf. the
    -- protobuf `xmlDetail` escape hatch).
    extra_detail     jsonb not null default '{}'::jsonb,

    created_at       timestamptz not null default now(),

    constraint cot_events_validity_interval check (stale_time >= start_time)
);

create index if not exists cot_events_uid_send_time_idx
    on cot_events (uid, send_time desc);

create index if not exists cot_events_send_time_idx
    on cot_events (send_time desc);

create index if not exists cot_events_owner_idx
    on cot_events (owner_id, send_time desc);

-- CoT semantics: the newest event for a uid supersedes earlier ones. This view
-- gives the current situational picture.
create or replace view cot_latest as
select distinct on (uid) *
from cot_events
order by uid, send_time desc;
