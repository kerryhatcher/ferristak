# Working on a Fly.io project

Portable guidance for agents. It is not auto-loaded as a context file; reference
it from the project's own `AGENTS.md` if you want it applied.

## Bias toward current docs

Fly.io changes. Before relying on memory for a command or a `fly.toml` key, check
the source: `fly <command> --help`, `fly docs`, <https://fly.io/docs/>, or the
files in `reference/`. The MCP catalog in `reference/fly-mcp-tools.md` is
generated from the installed flyctl and is the accurate tool list.

## Non-negotiables

- **Never print secrets.** `fly secrets list` shows names and digests only. Do not
  echo `DATABASE_URL`, tokens, or keys into logs, files, or the transcript.
- **Read before you change.** Establish state with read-only tools
  (`fly status`, `fly logs`, `fly machine list`, `fly releases`) before proposing
  or making a mutation.
- **Confirm destructive actions.** Destroying apps, machines, volumes or orgs, and
  changing secrets, all have real consequences. In interactive sessions these are
  confirmed for you; in headless sessions they are blocked unless explicitly
  allowed.
- **Keep `fly.toml` authoritative.** Prefer `fly deploy` over hand-editing
  machines; the next deploy reconciles machines to `fly.toml` and discards manual
  drift.

## Defaults worth following

- Migrations run from `[deploy] release_command`, exactly once per deploy — never
  on every machine's boot path.
- Health checks are cheap and dependency-light; put database readiness on a
  separate `/ready` route.
- The process binds `0.0.0.0:$PORT`; `internal_port` must match it.
- Secrets and non-secret `[env]` are separate concerns; only the latter belongs in
  Git.
- Every mutation is followed by a re-read that confirms the intended effect.

## When something is broken

Follow the `fly-troubleshooting` skill: `fly status` → `fly logs` →
`fly releases`, then the smallest safe action. Report evidence before acting.
