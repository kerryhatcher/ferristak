---
name: fly-databases
description: Connect a Fly.io app to Postgres, including Supabase and Fly Managed Postgres. Use when choosing a connection string, configuring pooling or prepared statements, using private networking, sizing the connection pool, or running migrations and backups for a database-backed Fly app.
---

# Databases from Fly.io

Two common shapes:

- **External Postgres (Supabase, Neon, RDS)** — simplest; the app dials out over
  the public internet. This repo uses Supabase.
- **Fly Managed Postgres (`fly mpg`)** — Fly runs the cluster, reachable over
  private networking.

Verify current behavior at <https://fly.io/docs/mpg/> and
<https://supabase.com/docs/guides/database/connecting-to-postgres>.

## Supabase connection strings

Supabase exposes three endpoints. Pick deliberately:

| Endpoint | Port | Use |
|---|---|---|
| Direct (`db.<ref>.supabase.co`) | 5432 | IPv6-only; fine from Fly if you have v6 egress |
| Supavisor **session** pooler (`aws-0-<region>.pooler.supabase.com`) | 5432 | default for a long-lived server |
| Supavisor **transaction** pooler | 6543 | serverless/short-lived; **no server-side prepared statements** |

For the ferristak API, use the **session pooler**. If you must use transaction
pooling, set `DB_DISABLE_PREPARED_STATEMENTS=true` — sqlx otherwise fails with
errors such as `prepared statement "sqlx_s_…" already exists`.

```
DATABASE_URL=postgresql://postgres.<ref>:<password>@aws-0-<region>.pooler.supabase.com:5432/postgres
```

Prefer the pooler over the IPv6-only direct host when you are unsure about Fly
egress. Region-match the pooler to your `primary_region` to cut latency.

## Pool sizing

Supabase's pooler caps concurrent clients per plan. Budget:

```
machines × DB_MAX_CONNECTIONS  <  pooler client limit
```

Five connections per machine is a good default. `min_machines_running = 0` apps
still open a pool per machine that is awake.

## Migrations

Run migrations exactly once per deploy, from `[deploy] release_command` — not in
each machine's boot path. This repo ships `ferristak migrate` for that purpose:

```toml
[deploy]
  release_command = "ferristak migrate"
```

The release command runs with app secrets, so `DATABASE_URL` is available. It
aborts the deploy on failure, leaving the previous release serving.

## Fly Managed Postgres

```bash
fly mpg create --name app-db --region iad     # cluster
fly mpg list
fly mpg status <id>
fly mpg connect <id>                          # psql
fly mpg backups list <id>
fly mpg proxy <id>                            # local tunnel, inspect from laptop
```

Machines reach it over `<app>.internal` / `.flycast`. Private networking avoids
public egress and TLS setup, but ties the app to a Fly org and region.

## Safety

- Never print a full `DATABASE_URL`; it contains the password. Reference the
  secret by name (`fly secrets list`) instead.
- Test destructive SQL against a branch or a `fly mpg proxy` session, not
  production. Supabase supports database branches on paid plans.
- Confirm backups exist before schema changes: `fly mpg backups list` (Fly) or
  the Supabase dashboard's backup/PITR page.
