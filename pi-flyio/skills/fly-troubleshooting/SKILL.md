---
name: fly-troubleshooting
description: Diagnose a failing or unreachable Fly.io app. Use when a deploy fails, health checks never pass, machines crash-loop or restart, the app is unreachable over HTTP, TLS certificates are pending, or you need to read logs, inspect a machine, or roll back.
---

# Troubleshooting Fly.io

Work from evidence, not guesses. Start with `fly logs` and
`fly machine status <id>`; escalate to a shell only when logs are insufficient.
Check current guidance at <https://fly.io/docs/monitoring/> and `fly doctor`.

## Triage order

1. **Is it deployed and healthy?** `fly status` (MCP `mcp__fly__fly_status`).
   Note each machine's state and health-check result.
2. **What does the app say?** `fly logs` (`mcp__fly__fly_logs`). The last lines
   before a restart usually name the cause.
3. **What happened at release time?** `fly releases`
   (`mcp__fly__fly_apps_releases`) and the deploy output — did the
   `release_command` fail?
4. **Is the environment sane?** `fly doctor`; `fly auth whoami`.

## Symptom → likely cause

| Symptom | Check |
|---|---|
| Health check never passes, rollout hangs | `internal_port` mismatch; app bound to `127.0.0.1`; health route errors |
| Machine restarts in a loop | crash before serving; missing secret; failed migration |
| `exit code 137` | OOM — `fly scale memory`, or fix the leak |
| App reachable on `.fly.dev` but not your domain | `fly certs list` / `fly certs check <host>`; DNS still propagating |
| 502 after a good deploy | no machine listening yet (cold start), or wrong port |
| Works locally, fails on Fly | missing secret, architecture (build `linux/amd64`), or `$PORT` ignored |
| Intermittent DB errors | pooler connection cap; transaction pooler + prepared statements |

## Commands and MCP tools

```bash
fly logs                    # mcp__fly__fly_logs
fly status                  # mcp__fly__fly_status
fly machine list            # mcp__fly__fly_machine_list
fly machine status <id>     # mcp__fly__fly_machine_status
fly releases                # mcp__fly__fly_apps_releases
fly checks list             # health checks
fly ssh console             # shell into a machine
fly console                 # throwaway machine with the same image/env
fly proxy 8080:8080         # local tunnel to a machine
fly doctor                  # local environment diagnosis
```

`fly console` is the safest way to reproduce a boot failure interactively: it
starts a fresh machine from the deployed image with the same secrets, so you can
run the entrypoint by hand.

## Reading logs well

- Filter to the crash window: `fly logs --app <app>` then reproduce, or use the
  MCP `fly_logs` tool.
- Application logs go to stdout/stderr of PID 1. If your process logs to a file,
  Fly will not show them.
- A `release_command` failure appears in the deploy output, not in machine logs.
  `fly logs` shows only running machines.

## Recover

- **Roll back:** pin the previous image from `fly releases` in `fly.toml`
  (`[build] image = "…"`) and `fly deploy`. There is no `fly rollback`.
- **Unstick a machine:** `fly machine restart <id>`, then `fly machine
  destroy <id>` if it is wedged; `fly deploy` recreates it from `fly.toml`.
- **Bad config:** fix `fly.toml`, redeploy. Editing a machine by hand diverges it
  from `fly.toml` and the next deploy overwrites your change.
