---
name: fly-deploy
description: Deploy or update an app on Fly.io. Use when writing or reviewing fly.toml or a Fly Dockerfile, configuring health checks, release commands, deploy strategies or zero-downtime rollouts, or running fly deploy and verifying that a release came up healthy.
---

# Deploying to Fly.io

Fly changes. Verify syntax against the live docs before relying on memory:
run `fly docs`, or fetch <https://fly.io/docs/launch/> /
<https://fly.io/docs/reference/configuration/>. The Fly MCP server
(`fly_status`, `fly_apps_releases`, `fly_logs`) shows real state.

## Preflight

1. Confirm tooling and identity: `fly version` and `fly auth whoami`
   (or `/fly-context`). The MCP tool `mcp__fly__fly_apps_list` lists owned apps.
2. Read `fly.toml` and the `Dockerfile`. If `fly.toml` is missing, `fly launch`
   generates one — review every generated value before deploying.
3. Confirm required secrets exist (see the `fly-secrets` skill). A missing
   `DATABASE_URL` or signing key fails at boot, not at deploy time.
4. Make sure the image listens on the port Fly routes to. Fly sets `PORT`
   (default `8080`) and expects the process bound to `0.0.0.0`, not `127.0.0.1`.

## fly.toml essentials

See `../../reference/fly-toml.md` for an annotated reference. The parts that
matter most:

- `app` and `primary_region` — deploy target and default region.
- `[build]` — Dockerfile path, build args, or a prebuilt `image`.
- `[env]` — non-secret configuration. Secrets never go here.
- `[http_service]` (or `[[services]]`) — `internal_port` **must** match the
  port the app listens on, and `force_https = true` is the usual choice.
- `[[http_service.checks]]` — the health check that gates a rollout. Point it at
  a cheap liveness route (the ferristak API exposes `/health`).
- `[deploy] release_command` — runs once per deploy in a temporary machine
  **with secrets available**, before machines update. Use it for migrations.
- `[[vm]]` — CPU/memory per machine. Right-size; 256 MB is often too small for a
  Rust release build's runtime working set once the DB pool is warm.

## Migrations and one-off tasks

Run schema migrations in `release_command`, never in every machine's boot path
(concurrent boots race). Example, matching this repo's binary:

```toml
[deploy]
  release_command = "ferristak migrate"
```

Set `RUN_MIGRATIONS=false` in `[env]` when the release command owns migrations.
A failing release command aborts the deploy and leaves the previous version
running, which is the behavior you want.

## Deploy and verify

```bash
fly deploy                       # build and roll out
fly status                       # machines, health, image
fly releases                     # history; is the new release live?
fly logs                         # stream application logs
fly checks list                  # health-check status per machine
```

Via MCP, prefer: `fly_status`, then `fly_apps_releases`, then `fly_logs`. Treat a
deploy as done only when health checks report passing on every machine.

For a local smoke test without touching production, `fly proxy 8080:8080` tunnels
to a machine, or `fly ssh console` opens a shell.

## Rollback

There is no single `fly rollback` command. Redeploy the previous release's image
by pinning it in `fly.toml` (`[build] image = "..."`) or redeploy the previous
Git revision with `fly deploy`. `fly releases` lists images per release.

## Pitfalls

- Wrong `internal_port` → health checks fail and the rollout never completes.
- Binding to `127.0.0.1` → the proxy cannot reach the app.
- Migrations on boot instead of `release_command` → duplicate/racing migrations.
- Health check that hits a DB-heavy route → flapping under load. Keep it cheap.
- Forgetting `force_https` → plaintext is served on the `.fly.dev` host.
