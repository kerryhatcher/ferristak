---
description: Plan and run a Fly.io deploy safely
argument-hint: "[app or environment]"
---
Deploy this project to Fly.io${1:+ (target: $1)}.

Follow the `fly-deploy` skill. Before changing anything:

1. Read `fly.toml`, the `Dockerfile`, and whatever the app reads from the
   environment (e.g. its config module and `.env.example`).
2. Report: app name, primary region, `internal_port`, health-check path,
   `release_command`, VM size, and the required secret names. Flag any mismatch
   between the port the app binds and `internal_port`.
3. Check current state with `fly status` (or `mcp__fly__fly_status`) and confirm
   the required secrets exist by name (`fly secrets list`).

Then deploy, watch the release output, and verify that health checks pass on
**every** machine. If anything fails, stop and report the exact error with the
relevant log lines instead of retrying blindly. End with the release version,
machine states, and the URL.
