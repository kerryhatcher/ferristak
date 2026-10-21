/**
 * pi-flyio — Fly.io integration for Pi.
 *
 * What it does:
 *  - Registers Fly's official MCP server (`fly mcp server`) over stdio, so the
 *    model can inspect and change Fly resources.
 *  - Confirms every mutating Fly MCP tool. The Fly server marks all of its
 *    tools `destructiveHint: true` (even reads), so we allow-list reads and ask
 *    before anything else.
 *  - Adds `/fly-context`, `/fly-status`, and `/fly-doctor` commands, and shows
 *    the app from `fly.toml` in the status line.
 *
 * Configuration (environment):
 *  - `FLYCTL_BIN`              absolute path to flyctl/fly.
 *  - `FLY_MCP_DISABLE=1`       do not register the MCP server.
 *  - `FLY_MCP_ASSUME_YES=1`    skip mutation confirmations (CI / headless).
 *  - `FLY_MCP_EXPOSURE`        codemode | deferred | direct | hidden
 *                              (default: deferred).
 *
 * The flag `--no-fly` disables the MCP server for one run.
 */

import { existsSync, readFileSync } from "node:fs";
import { homedir } from "node:os";
import { delimiter, join } from "node:path";
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";

const SERVER_NAME = "fly";
const TOOL_PREFIX = `mcp__${SERVER_NAME}__`;

/** Normalised Fly MCP tool names that only read state. Everything else is confirmed. */
const READ_ONLY_TOOLS = new Set<string>([
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
]);

type Exposure = "codemode" | "deferred" | "direct" | "hidden";

function findOnPath(name: string): string | undefined {
  const path = process.env.PATH ?? "";
  for (const dir of path.split(delimiter)) {
    if (dir && existsSync(join(dir, name))) {
      // Return the bare name so the child process keeps using PATH.
      return name;
    }
  }
  return undefined;
}

function resolveFlyctl(): string | undefined {
  const override = process.env.FLYCTL_BIN?.trim();
  if (override) return override;

  for (const name of ["flyctl", "fly"]) {
    const found = findOnPath(name);
    if (found) return found;
  }

  for (const candidate of [
    join(homedir(), ".fly", "bin", "flyctl"),
    join(homedir(), ".fly", "bin", "fly"),
  ]) {
    if (existsSync(candidate)) return candidate;
  }
  return undefined;
}

function resolveExposure(): Exposure {
  const value = process.env.FLY_MCP_EXPOSURE;
  if (value === "codemode" || value === "deferred" || value === "direct" || value === "hidden") {
    return value;
  }
  return "deferred";
}

interface FlyToml {
  path: string;
  app?: string;
  region?: string;
}

function readFlyToml(cwd: string): FlyToml | undefined {
  const path = join(cwd, "fly.toml");
  if (!existsSync(path)) return undefined;
  const text = readFileSync(path, "utf8");
  return {
    path,
    app: text.match(/^\s*app\s*=\s*"([^"]+)"/m)?.[1],
    region: text.match(/^\s*primary_region\s*=\s*"([^"]+)"/m)?.[1],
  };
}

function shorten(text: string, maxLines = 16): string {
  const lines = text
    .trim()
    .split(/\r?\n/)
    .filter((line) => line.trim().length > 0);
  if (lines.length === 0) return "(no output)";
  if (lines.length <= maxLines) return lines.join("\n");
  return [...lines.slice(0, maxLines), `… (${lines.length - maxLines} more lines omitted)`].join("\n");
}

export default function flyio(pi: ExtensionAPI) {
  pi.registerFlag("no-fly", {
    description: "Disable the Fly.io MCP server for this run",
    type: "boolean",
    default: false,
  });

  const disabled = process.env.FLY_MCP_DISABLE === "1" || pi.getFlag("no-fly") === true;
  const flyctl = resolveFlyctl();

  if (flyctl && !disabled) {
    pi.registerMcpServer(SERVER_NAME, {
      command: flyctl,
      args: ["mcp", "server"],
      exposure: resolveExposure(),
      description:
        "Inspect and manage Fly.io apps, machines, volumes, secrets, IPs, certificates and organizations.",
      timeout: 120,
    });
  }

  // Confirm mutating Fly tools. Reads pass through; headless sessions block by default.
  pi.on("tool_call", async (event, ctx) => {
    if (!event.toolName.startsWith(TOOL_PREFIX)) return undefined;

    const tool = event.toolName.slice(TOOL_PREFIX.length);
    if (READ_ONLY_TOOLS.has(tool)) return undefined;
    if (process.env.FLY_MCP_ASSUME_YES === "1") return undefined;

    const detail = `${tool}\n${JSON.stringify(event.input ?? {}, null, 2)}`;

    if (!ctx.hasUI) {
      return {
        block: true,
        reason: `Fly.io mutation ${tool} needs confirmation, but this session has no UI. Set FLY_MCP_ASSUME_YES=1 to allow it.`,
      };
    }

    const approved = await ctx.ui.confirm("Run Fly.io mutation?", detail);
    return approved ? undefined : { block: true, reason: `User declined ${tool}` };
  });

  pi.registerCommand("fly-context", {
    description: "Show the Fly app, region and auth context for this project",
    handler: async (_args, ctx) => {
      const toml = readFlyToml(ctx.cwd);
      const lines: string[] = [];

      lines.push(toml?.app ? `app:    ${toml.app}` : "app:    (no fly.toml in this directory)");
      if (toml?.region) lines.push(`region: ${toml.region}`);
      if (toml?.path) lines.push(`config: ${toml.path}`);

      if (flyctl) {
        const who = await pi.exec(flyctl, ["auth", "whoami"], { cwd: ctx.cwd, timeout: 20_000 });
        const identity = who.code === 0 ? who.stdout.trim().split(/\r?\n/)[0] : undefined;
        lines.push(`auth:   ${identity || "(not authenticated)"}`);
      } else {
        lines.push("auth:   (flyctl not found on PATH)");
      }

      ctx.ui.notify(lines.join("\n"), "info");
    },
  });

  pi.registerCommand("fly-status", {
    description: "Run `fly status` for the current project",
    handler: async (_args, ctx) => {
      if (!flyctl) {
        ctx.ui.notify("flyctl was not found on PATH", "error");
        return;
      }
      const result = await pi.exec(flyctl, ["status"], { cwd: ctx.cwd, timeout: 60_000 });
      ctx.ui.notify(
        shorten(result.stdout || result.stderr),
        result.code === 0 ? "info" : "error",
      );
    },
  });

  pi.registerCommand("fly-doctor", {
    description: "Run `fly doctor` to diagnose the local Fly environment",
    handler: async (_args, ctx) => {
      if (!flyctl) {
        ctx.ui.notify("flyctl was not found on PATH", "error");
        return;
      }
      const result = await pi.exec(flyctl, ["doctor"], { cwd: ctx.cwd, timeout: 90_000 });
      ctx.ui.notify(
        shorten(result.stdout || result.stderr),
        result.code === 0 ? "info" : "warning",
      );
    },
  });

  pi.on("session_start", async (_event, ctx) => {
    const toml = readFlyToml(ctx.cwd);
    if (toml?.app) ctx.ui.setStatus("fly", `fly: ${toml.app}`);

    if (!flyctl && !disabled) {
      ctx.ui.notify(
        "pi-flyio: flyctl/fly was not found, so the Fly MCP server is not registered. Install flyctl or set FLYCTL_BIN.",
        "warning",
      );
    }
  });

  pi.on("session_shutdown", async (_event, ctx) => {
    ctx.ui.setStatus("fly", undefined);
  });
}
