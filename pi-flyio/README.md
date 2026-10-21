# pi-flyio

A [Pi](https://pi.dev) package for working on **Fly.io** projects. It wires up
Fly's official MCP server, teaches agents the platform's sharp edges, and guards
destructive operations.

## What's inside

| Path | What it provides |
|---|---|
| `extensions/flyio.ts` | Registers `fly mcp server` (stdio), confirms mutations, adds `/fly-context`, `/fly-status`, `/fly-doctor`, and a `fly:` status line. |
| `skills/` | `fly-deploy`, `fly-machines`, `fly-secrets`, `fly-databases`, `fly-troubleshooting`, `fly-mcp`. Loaded on demand. |
| `prompts/` | `/fly-deploy`, `/fly-triage`, `/fly-review`. |
| `reference/` | `fly-toml.md`, `fly-cli.md`, and a generated `fly-mcp-tools.md` catalog. |
| `scripts/gen-fly-mcp-catalog.py` | Regenerates the MCP catalog from the installed flyctl. |
| `AGENTS.md` | Portable guidance for agents touching a Fly project. |

## Install

As a package (discovers the extension, skills and prompts):

```bash
pi install ./pi-flyio
```

For one run without installing:

```bash
pi -e ./pi-flyio/extensions/flyio.ts
```

The extension is self-contained; skills and prompts only load when the package is
installed (or their paths are added to settings). In this repository,
`.pi/settings.json` already declares `./pi-flyio` as a project package, so it
loads after project trust is granted.

## Requirements

- `flyctl` (the `fly` command) on `PATH`, or `FLYCTL_BIN` set to its path.
- An authenticated flyctl session (`fly auth login`); MCP calls act as that identity.

## Configuration

| Environment variable | Effect |
|---|---|
| `FLYCTL_BIN` | Path to the flyctl/fly binary. Detected from `PATH`, then `~/.fly/bin`. |
| `FLY_MCP_DISABLE=1` | Do not register the MCP server. |
| `FLY_MCP_ASSUME_YES=1` | Skip mutation confirmations (headless/CI only). |
| `FLY_MCP_EXPOSURE` | `deferred` (default) · `codemode` · `direct` · `hidden`. |

The `--no-fly` flag disables the MCP server for a single run.

## Commands and prompts

| Command | Purpose |
|---|---|
| `/fly-context` | App, region, config path and authenticated identity. |
| `/fly-status` | Runs `fly status` for the current project. |
| `/fly-doctor` | Runs `fly doctor`. |
| `/fly-deploy [target]` | Plan and run a safe deploy. |
| `/fly-triage [app]` | Diagnose an unhealthy app from logs and state. |
| `/fly-review [focus]` | Review `fly.toml` + Dockerfile for production readiness. |

Skills are also invocable explicitly, e.g. `/skill:fly-deploy`.

## MCP tools

The Fly server exposes ~60 tools under the `mcp__fly` namespace. With the default
`deferred` exposure, discover them with `tool_search`, then call them:

```text
tool_search("fly machine list")
→ call mcp__fly__fly_machine_list { "app": "ferristak" }
```

Reads run freely; mutations prompt for confirmation. See
[`reference/fly-mcp-tools.md`](reference/fly-mcp-tools.md) for the full catalog and
which tools are treated as read-only.

## Security

- The extension runs `flyctl` with your credentials and can change or destroy
  real infrastructure. Review it before enabling it in a shared environment.
- Every non-read tool is confirmed interactively, or blocked when there is no UI.
- Secrets are never read or printed. `fly secrets list` reveals names and digests
  only, and the skills instruct agents to keep it that way.
- The Fly MCP server itself is marked experimental upstream; the tool set can
  change between flyctl releases.

## Regenerating the catalog

```bash
python3 scripts/gen-fly-mcp-catalog.py
```

Commit the result after upgrading flyctl so agents see the accurate tool set.
