-- 0004_hardening.sql
--
-- Follow-ups from the interface/security review:
--   1. Make the CoT grammar checks ASCII. Postgres `\w` is locale-dependent and
--      can accept non-ASCII letters; the XML Schema `xs:pattern` (and the
--      application regexes) are ASCII `[A-Za-z0-9_]`.
--   2. Enforce the event time window (`start <= time <= stale`) in the database.
--   3. Resolve the "RLS enabled, no policies" state. Supabase enables RLS on new
--      public tables and grants anon/authenticated access, expecting PostgREST
--      policies to gate them. This service does not use PostgREST, only the
--      server role connects, and authorization is enforced in the application.
--      With no policies, RLS is a no-op for the table owner but silently blocks
--      any future least-privilege application role. Revoke the PostgREST roles
--      and disable RLS so table grants, not RLS, control access.

-- 1. ASCII grammar (replace the locale-dependent `\w` constraints).
alter table public.cot_events drop constraint if exists cot_events_event_type_check;
alter table public.cot_events
    add constraint cot_events_event_type_ascii_check
    check (event_type ~ '^[A-Za-z0-9_]+(-[A-Za-z0-9_]+)*(;[^;]*)?$');

alter table public.cot_events drop constraint if exists cot_events_how_check;
alter table public.cot_events
    add constraint cot_events_how_ascii_check
    check (how ~ '^[A-Za-z0-9_](-[A-Za-z0-9_]+)*$');

alter table public.cot_events drop constraint if exists cot_events_qos_check;
alter table public.cot_events
    add constraint cot_events_qos_ascii_check
    check (qos is null or qos ~ '^[0-9]-[A-Za-z0-9_]-[A-Za-z0-9_]$');

-- 2. Time window. Supersedes the older `stale >= start` check, which is kept.
alter table public.cot_events drop constraint if exists cot_events_time_window;
alter table public.cot_events
    add constraint cot_events_time_window
    check (send_time >= start_time and send_time <= stale_time);

-- 3. PostgREST roles off, RLS off (application-enforced authorization).
do $$
begin
    if exists (select 1 from pg_roles where rolname = 'anon') then
        execute 'revoke all on table public.users, public.cot_events from anon';
    end if;
    if exists (select 1 from pg_roles where rolname = 'authenticated') then
        execute 'revoke all on table public.users, public.cot_events from authenticated';
    end if;
end
$$;

alter table public.users disable row level security;
alter table public.cot_events disable row level security;

-- sqlx's bookkeeping table gets RLS from the same Supabase default; a
-- least-privilege role running migrations would be blocked without a policy.
alter table if exists public._sqlx_migrations disable row level security;
