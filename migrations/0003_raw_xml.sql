-- 0003_raw_xml.sql
--
-- Preserve the original CoT `<event>…</event>` element for events submitted as
-- XML, so that XML round-trips are lossless: unknown detail children, unknown
-- attributes, attribute casing (`releasableto` vs `releasableTo`) and element
-- ordering all survive. The typed columns remain a searchable index only.
--
-- This replaces the unused `extra_detail` JSONB column. TAK's protobuf
-- `xmlDetail` is raw XML children, not JSON, so the old column could never be
-- re-emitted as CoT and silently lost data.

-- `cot_latest` was defined as `select *`, so it holds a dependency on every
-- column. Drop it first, then recreate it after the column change so it picks
-- up `raw_xml`.
drop view if exists cot_latest;

alter table cot_events
    add column if not exists raw_xml text not null default '';

alter table cot_events
    drop column if exists extra_detail;

create or replace view cot_latest as
select distinct on (uid) *
from cot_events
order by uid, send_time desc;
