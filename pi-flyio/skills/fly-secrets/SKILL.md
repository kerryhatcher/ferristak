---
name: fly-secrets
description: Manage Fly.io secrets and application configuration. Use when setting, rotating, listing or removing secrets, wiring environment variables into an app, debugging a missing or wrong secret, or deciding what belongs in fly.toml env versus a secret.
---

# Fly.io secrets and configuration

Secrets are encrypted at rest, injected as environment variables at boot, and
**changing one triggers a new release** (machines restart). Verify current flags
with `fly secrets --help` and <https://fly.io/docs/apps/secrets/>.

## Decide: env or secret

| Value | Where | Why |
|---|---|---|
| `DATABASE_URL`, API keys, `JWT_SECRET` | `fly secrets` | encrypted, redacted from output |
| `RUST_LOG`, feature flags, `PORT` | `[env]` in `fly.toml` | non-sensitive, reviewable in Git |

Never put credentials in `fly.toml`, the Dockerfile, or `[env]`.

## Set and rotate

```bash
fly secrets set JWT_SECRET="$(openssl rand -base64 48)"
fly secrets set DATABASE_URL='postgresql://…' JWT_SECRET='…'   # several at once
fly secrets import < .env.production                            # bulk
fly secrets list                                                # names + digests, never values
fly secrets unset OLD_KEY
```

Via MCP: `mcp__fly__fly_secrets_list` (read-only) then `mcp__fly__fly_secrets_set`
/ `_unset` / `_deploy` (confirmed). `fly-secrets-set` restarts machines; plan for
the associated brief disruption unless the app tolerates a rolling restart.

Stage without restarting, then apply, when you need a controlled cutover:

```bash
fly secrets set NEW_KEY=… --stage
fly secrets deploy
```

In the ferristak stack the required secrets are `DATABASE_URL` and `JWT_SECRET`
(≥ 32 bytes). The API refuses to start without them.

## Handling secret values safely

- Never print secret values, and do not echo them into logs or tool output.
  `fly secrets list` shows digests only — that is intentional; do not try to
  recover values from it.
- Generate on the machine that sets them, not by pasting into a transcript:
  `fly secrets set JWT_SECRET="$(openssl rand -base64 48)"`.
- Prefer one secret per concern so rotation is isolated.

## Debugging missing config

1. `fly secrets list` — is the name present? (Values are not shown.)
2. `fly ssh console` then `env | grep -v '^FLY_' | cut -d= -f1` to see **names**
   present inside the machine, without revealing values.
3. `fly logs` — boot errors usually name the missing variable.
4. Remember `release_command` runs with secrets available; a command that works
   there but not on boot usually means the value is only set for the release
   environment, which does not exist — secrets are app-wide.
