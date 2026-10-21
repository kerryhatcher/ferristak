---
description: Review fly.toml and the Dockerfile for production readiness
argument-hint: "[focus]"
---
Review this project's Fly.io setup for production readiness${1:+ (focus: $1)}.

Read `fly.toml` and the `Dockerfile` and check at least:

- `internal_port` matches the port the process binds (`0.0.0.0:$PORT`, not
  `127.0.0.1`),
- a health check exists, is cheap, and does not depend on the database,
- migrations run via `[deploy] release_command` exactly once per deploy, not on
  every machine boot,
- secrets and `[env]` are separated correctly and no credential is committed,
- regions and VM size are sensible, and `auto_stop_machines` /
  `min_machines_running` are intentional,
- the image is multi-stage, minimal, runs as non-root, and builds `linux/amd64`.

Report concrete problems with file/line references and a suggested change for
each. Rank by risk. Do not edit files unless I ask.
