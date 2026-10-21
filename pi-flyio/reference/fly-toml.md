# fly.toml reference

A working reference for the keys that matter in practice, using this repo's
`fly.toml` as the worked example. This is a guide, not the spec — the authoritative
reference is <https://fly.io/docs/reference/configuration/>. `fly config validate`
checks a file; `fly config show` prints the resolved configuration.

## Annotated example (ferristak)

```toml
app = "ferristak"                 # globally unique; also the *.fly.dev hostname
primary_region = "iad"            # default region for new machines

[build]                           # omit to auto-detect a Dockerfile

[env]                             # non-secret config; change is not a release
  BIND_ADDR = "0.0.0.0:8080"
  RUN_MIGRATIONS = "false"        # migrations run via release_command instead
  RUST_LOG = "info,ferristak=info,tower_http=info"

[http_service]
  internal_port = 8080            # MUST equal the port the process binds
  force_https = true              # redirect HTTP -> HTTPS
  auto_stop_machines = "suspend"  # scale to zero when idle ("off" | "stop" | "suspend")
  auto_start_machines = true
  min_machines_running = 0        # raise to keep capacity warm
  processes = ["app"]

  [[http_service.checks]]
    interval = "15s"
    timeout = "2s"
    grace_period = "10s"          # give the boot time before failing checks
    method = "GET"
    path = "/health"              # cheap liveness; do not touch the DB here

[deploy]
  release_command = "ferristak migrate"   # runs once per deploy, with secrets

[[vm]]
  size = "shared-cpu-1x"
  memory = "512mb"
```

## Sections

### `[build]`

```toml
[build]
  dockerfile = "Dockerfile"       # default
  build-target = "runtime"        # multi-stage target
  [build.args]
    CARGO_PROFILE = "release"
```

Or deploy a prebuilt image (also how you pin a rollback):

```toml
[build]
  image = "registry.fly.io/ferristak:deployment-01J…"
```

### `[env]` and secrets

`[env]` is plaintext and belongs in Git. Anything sensitive goes to
`fly secrets set`. `[env]` changes are applied without a new image; secret
changes trigger a release.

### `[http_service]` vs `[[services]]`

`[http_service]` is the modern, HTTP-focused form (TLS termination, concurrency
limits, health checks, autostop). `[[services]]` is the older, more general form
that can expose raw TCP/UDP. Do not define both for the same port.

```toml
[http_service]
  internal_port = 8080
  force_https = true
  [http_service.concurrency]
    type = "requests"             # or "connections"
    soft_limit = 200
    hard_limit = 250
```

### Health checks

```toml
[[http_service.checks]]
  interval = "15s"
  timeout = "2s"
  grace_period = "10s"
  method = "GET"
  path = "/health"
  # [[http_service.checks.headers]]
  #   X-Health-Check = "fly"
```

A failing check blocks the rollout and marks the machine unhealthy. Keep the
route dependency-light; a DB-backed readiness probe belongs at `/ready`, not
here.

### `[deploy] release_command`

Runs once per deploy in a one-off machine built from the new image, **with app
secrets**, before any machine is updated. A non-zero exit aborts the deploy and
leaves the previous release serving. Use it for migrations and one-off setup —
never for per-boot work.

### `[[vm]]` and scaling

```toml
[[vm]]
  size = "shared-cpu-1x"          # or "performance-2x", "shared-cpu-2x", …
  memory = "512mb"
  cpus = 2                        # some sizes allow explicit cpu count
```

`fly scale count N` and `fly scale vm <size>` override at runtime; keep
`fly.toml` in sync so the next deploy does not silently revert them.

### `[mounts]` (Volumes)

```toml
[[mounts]]
  source = "data"
  destination = "/data"
```

Volumes are region-bound and attach to one machine at a time. Back them up with
`fly volumes snapshots`.

### `[processes]` and release machines

```toml
[processes]
  app = "ferristak serve"
  worker = "ferristak worker"
```

Multiple process groups run in the same app/image and are scaled independently.
`[deploy] release_command` is a one-off, not a process group.

## Validation workflow

```bash
fly config validate        # parse and validate fly.toml
fly config show            # resolved config as Fly sees it
fly config save --app <app>  # pull the live config back into fly.toml
```

After editing, always `fly config validate` before `fly deploy`.
