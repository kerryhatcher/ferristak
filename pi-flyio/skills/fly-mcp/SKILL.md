---
name: fly-mcp
description: Use the Fly.io MCP server tools from Pi. Use when you need to inspect or change Fly apps, machines, secrets, volumes, certificates or organizations and need the correct mcp__fly tool names, how to load deferred tools, and which calls require confirmation.
---

# Using the Fly.io MCP tools

`pi-flyio` registers Fly's official MCP server (`fly mcp server`) under the
namespace `mcp__fly`. The complete, generated catalog lives in
`../../reference/fly-mcp-tools.md` (60 tools at the time of writing).

## Find and call a tool

The server uses **`deferred` exposure** by default, so its tools are not declared
to the model. Load what you need, then call it:

1. `tool_search` with a query, e.g. `tool_search("fly machine list")`.
2. The matching `mcp__fly__…` tools become callable on the next request.
3. Call the tool directly with its arguments.

Set `FLY_MCP_EXPOSURE=codemode` to reach the tools from codemode scripts instead
(`searchTools`, `describeTool`, `executeTool`). `direct` declares all 60 tools to
the model at once (large context cost; avoid). `hidden` disables them.

## Names and arguments

The server names tools like `fly-apps-list`. Pi replaces every character other
than letters, digits and `_` with `_`, so the callable name is
`mcp__fly__fly_apps_list`. Consult the catalog when unsure.

MCP calls have **no `fly.toml` context**: pass `app`/`name` explicitly, unlike the
CLI which infers the app from the working directory.

## Safety and confirmations

`pi-flyio` confirms every mutating Fly tool. The Fly server marks *all* of its
tools `destructiveHint: true` — even `fly-apps-list` — so the package instead
allow-lists the read-only tools and asks before everything else.

- **Read-only (no prompt):** `fly_status`, `fly_logs`, `fly_machine_list`,
  `fly_machine_status`, `fly_apps_list`, `fly_apps_releases`, `fly_secrets_list`,
  `fly_volumes_list`, `fly_platform_regions`, and similar list/show tools.
- **Confirmed:** `fly_apps_destroy`, `fly_apps_create`, `fly_apps_restart`,
  `fly_machine_stop|kill|destroy|restart|clone|update|exec`,
  `fly_secrets_set|unset|deploy`, `fly_volumes_create|destroy|extend`,
  `fly_ips_allocate_*`, `fly_orgs_*` mutations, and the rest.

In a headless/CI session there is no dialog, so mutations are blocked. Set
`FLY_MCP_ASSUME_YES=1` only when you truly intend them.

## Working style

1. **Read first.** Establish state with read-only tools before proposing a change.
2. **One mutation at a time**, then re-read to confirm the effect.
3. **Secrets stay secret.** `fly_secrets_list` returns names and digests, never
   values; never attempt to recover values through other tools or logs.
4. **No resources.** The Fly server exposes tools only; `list_mcp_resources` has
   nothing to return.

## Troubleshooting the connection

- `/mcp` (or `pi mcp list`) shows connection state and errors.
- The server authenticates with the existing `fly auth` session. Verify with
  `/fly-context` or `mcp__fly__fly_orgs_list`.
- If tools are missing, the flyctl version changed the tool set: regenerate the
  catalog with `python3 scripts/gen-fly-mcp-catalog.py`.
- Disable for one run with `pi --no-fly`, or for good with `FLY_MCP_DISABLE=1`.
