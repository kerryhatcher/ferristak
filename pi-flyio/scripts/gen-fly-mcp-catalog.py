#!/usr/bin/env python3
"""Regenerate reference/fly-mcp-tools.md from the live `fly mcp server`.

The Fly MCP server is experimental and its tool set changes between flyctl
releases. Run this after upgrading flyctl so agents see the real catalog.

Usage:
    python3 scripts/gen-fly-mcp-catalog.py [output.md]
"""

from __future__ import annotations

import json
import os
import subprocess
import sys

# Keep in sync with READ_ONLY_TOOLS in extensions/flyio.ts (normalised names).
READ_ONLY = {
    "fly_apps_list",
    "fly_apps_releases",
    "fly_certs_check",
    "fly_certs_list",
    "fly_certs_show",
    "fly_ips_list",
    "fly_logs",
    "fly_machine_egress_ip_list",
    "fly_machine_leases_view",
    "fly_machine_list",
    "fly_machine_status",
    "fly_orgs_list",
    "fly_orgs_show",
    "fly_platform_regions",
    "fly_platform_status",
    "fly_platform_vm_sizes",
    "fly_secrets_list",
    "fly_status",
    "fly_volumes_list",
    "fly_volumes_show",
    "fly_volumes_snapshots_list",
}


def flyctl_bin() -> str:
    return os.environ.get("FLYCTL_BIN") or "fly"


def probe() -> tuple[str, list[dict]]:
    messages = [
        {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": {"name": "pi-flyio-catalog", "version": "0"},
            },
        },
        {"jsonrpc": "2.0", "method": "notifications/initialized"},
        {"jsonrpc": "2.0", "id": 2, "method": "tools/list"},
    ]
    payload = "\n".join(json.dumps(message) for message in messages) + "\n"
    proc = subprocess.run(
        [flyctl_bin(), "mcp", "server"],
        input=payload,
        capture_output=True,
        text=True,
        timeout=90,
    )
    tools: list[dict] = []
    for line in proc.stdout.splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            obj = json.loads(line)
        except json.JSONDecodeError:
            continue
        if obj.get("id") == 2:
            tools = obj.get("result", {}).get("tools", [])
    version = subprocess.run(
        [flyctl_bin(), "version"], capture_output=True, text=True, timeout=30
    ).stdout.strip()
    return version, tools


def normalise(name: str) -> str:
    return "mcp__fly__" + name.replace("-", "_")


def escape(text: str | None) -> str:
    return (text or "").replace("|", "\\|").replace("\n", " ").strip()


def main() -> int:
    default_out = os.path.join(
        os.path.dirname(os.path.abspath(__file__)), "..", "reference", "fly-mcp-tools.md"
    )
    out_path = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else default_out)

    version, tools = probe()
    if not tools:
        print("error: the MCP server returned no tools", file=sys.stderr)
        return 1

    lines = [
        "# Fly.io MCP tool catalog",
        "",
        f"> Generated from the live `fly mcp server` ({version}). Do not edit by hand;",
        "> regenerate with `python3 scripts/gen-fly-mcp-catalog.py`.",
        "",
        f"The server exposes **{len(tools)} tools**. Pi registers it under the namespace",
        "`mcp__fly`, so a tool appears as `mcp__fly__<name>` (characters other than letters,",
        "digits and `_` become `_`).",
        "",
        "`Access` is this package's policy, not the server's. The Fly server marks every tool",
        "`destructiveHint: true`, so `pi-flyio` allow-lists reads and confirms everything else.",
        "Every tool also counts as `openWorldHint: true`.",
        "",
        "| MCP tool | Access | Description |",
        "|---|---|---|",
    ]
    for tool in sorted(tools, key=lambda item: item["name"]):
        name = tool["name"]
        access = "read-only" if name.replace("-", "_") in READ_ONLY else "confirmed"
        lines.append(f"| `{normalise(name)}` | {access} | {escape(tool.get('description'))} |")
    lines.append("")

    with open(out_path, "w", encoding="utf-8") as handle:
        handle.write("\n".join(lines) + "\n")

    print(f"wrote {out_path} ({len(tools)} tools)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
