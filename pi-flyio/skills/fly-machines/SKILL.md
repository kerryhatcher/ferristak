---
name: fly-machines
description: Manage Fly Machines, regions, volumes and scaling. Use when starting, stopping, restarting, replacing or cloning machines, right-sizing CPU/memory, adding or moving regions, attaching Volumes, or debugging a machine that will not stay up.
---

# Fly Machines, regions and volumes

A Fly app is a set of **Machines** (Firecracker VMs) running one image. `fly.toml`
and `fly deploy` create/update them, but day-to-day operations act on individual
machines. Verify current flags with `fly machine --help` and
<https://fly.io/docs/machines/>.

## Read state first

Via MCP (read-only, no confirmation):

- `mcp__fly__fly_machine_list` — every machine, state, region, image.
- `mcp__fly__fly_machine_status` — one machine in detail.
- `mcp__fly__fly_status` — app-level summary.

CLI equivalents: `fly machine list`, `fly status`.

## Lifecycle

```bash
fly machine start   <id>
fly machine stop    <id>      # graceful; keeps the machine
fly machine restart <id>
fly machine suspend <id>      # stop without losing the rootfs; wakes on traffic
fly machine kill    <id>      # hard stop
fly machine destroy <id>      # remove (irreversible; volumes are separate)
fly machine clone   <id> --region <region>
fly machine update  <id> --vm-memory 512 --vm-cpus 2
```

Mutating calls (`fly_machine_stop`, `_destroy`, `_clone`, `_update`, …) require
confirmation in Pi. Prefer `fly deploy` over hand-editing machines: it keeps
`fly.toml` and running machines consistent.

## Scaling

- **Horizontal:** `fly scale count 3`, or `[[vm]]` / `[http_service]` autoscaling
  with `auto_stop_machines` and `min_machines_running`.
- **Vertical:** `fly scale vm shared-cpu-2x` and `fly scale memory 1024`.
- `auto_stop_machines = "suspend"` + `min_machines_running = 0` scales idle apps
  to zero; expect a cold-start on the first request.

Do not scale above what the database can serve. Supabase's pooler caps client
connections; keep `DB_MAX_CONNECTIONS × machines` under that limit.

## Regions

```bash
fly platform regions          # available regions (MCP: fly_platform_regions)
fly scale count 2 --region sin
fly machine clone <id> --region fra
```

Put machines near users and near the database. Attach a Volume in the same region
as the machine that uses it — volumes are region-bound and cannot move.

## Volumes

```bash
fly volumes list                          # MCP: fly_volumes_list
fly volumes create data --region iad --size 1
fly volumes extend <id> --size 5
fly volumes snapshots list <id>           # backups
```

Mount them in `[mounts]` in `fly.toml`. One volume is attached to one machine at
a time, so a volume-backed app does not get free horizontal scaling. Treat
volumes as state to back up, not as a database.

## Debugging a flapping machine

1. `fly logs` / `mcp__fly__fly_logs` — the real error is usually the last lines
   before the restart.
2. `fly machine status <id>` — exit code and event history.
3. `fly ssh console -s` — shell into a running machine; `fly console` for a
   throwaway one with the same image/env.
4. OOM (`exit code 137`) → raise `fly scale memory` or fix the leak.
