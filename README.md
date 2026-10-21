# ferristak

A Rust-based **Team Awareness Kit** API — a small, production-shaped HTTP service
backed by [Supabase](https://supabase.com/) Postgres and deployed to
[Fly.io](https://fly.io/).

Situational-awareness data is modelled as **Cursor-on-Target (CoT) events**, the
public MITRE/MIL-STD-6090 XML standard used across the TAK ecosystem. Events can
be submitted and retrieved as canonical CoT XML or as an equivalent JSON shape.

## Stack

| Concern            | Choice                                                   |
| ------------------ | -------------------------------------------------------- |
| HTTP framework     | [`axum`](https://github.com/tokio-rs/axum) 0.8           |
| Async runtime      | [`tokio`](https://tokio.rs/)                             |
| Database           | Postgres via [`sqlx`](https://github.com/launchbadge/sqlx) (runtime-checked queries, no build-time DB) |
| Migrations         | sqlx, applied at runtime from `./migrations`             |
| CoT XML            | [`quick-xml`](https://github.com/tafia/quick-xml) serde   |
| Auth               | Argon2id passwords + HS256 JWTs (`argon2`, `jsonwebtoken`) |
| Validation         | schema-driven validation in the domain layer             |
| Observability      | `tracing` + `tracing-subscriber`, `tower-http` tracing   |
| Deployment         | Multi-stage Dockerfile + Fly.io (`fly.toml`)             |

## Cursor-on-Target alignment

The data model is derived from the reference schemas in `docs/sources/atak-civ`:

- **CoT Base-Event Schema 2.0** (`event`, `point`, `detail`) — MITRE, updated for
  MIL-STD-6090 (`access`, `caveat`, `releasableto`).
- **Detail sub-schemas** — `contact`, `__group`, `status`, `track`, `takv`,
  `precisionlocation`, `remarks`.
- **Protobuf definitions** (`takproto/*.proto`) — strongly-typed detail plus the
  `xmlDetail` escape hatch. We mirror it by preserving the original `<event>`
  element verbatim in `raw_xml` and treating the typed columns as an index.

Mapping:

| CoT XML                                   | Rust type / column                              |
| ----------------------------------------- | ----------------------------------------------- |
| `<event version type access uid time …>`  | `CotEvent` / `cot_events` (`cot_version`, `event_type`, `send_time`, …) |
| `<event caveat releasableto qos opex how>`| scalar columns                                  |
| `<point lat lon hae ce le>`               | `Point` (`lat`, `lon`, `hae`, `ce`, `le`)       |
| `<contact callsign endpoint>`             | `Contact` (`contact_callsign`, `contact_endpoint`) |
| `<__group name role>`                     | `Group` (`group_name`, `group_role`)            |
| `<status battery readiness>`              | `Status`                                        |
| `<track speed course slope>`              | `Track`                                         |
| `<takv device platform os version>`       | `Takv`                                          |
| `<precisionlocation geopointsrc altsrc>`  | `PrecisionLocation`                             |
| `<remarks>`                               | `remarks`                                       |
| original event element (unmapped detail, attributes, ordering) | `raw_xml` (text) |

Schema rules are enforced twice — in `NewCotEvent::validate()` and as database
`check` constraints:

- `type` matches `\w+(-\w+)*(;[^;]*)?`, `how` matches `\w(-\w+)*`, `qos`
  matches `\d-\w-\w` (the XSD `xs:pattern`s).
- `lat ∈ [-90, 90]`, `lon ∈ [-180, 180]`, `start <= stale`, `battery ∈ [0, 100]`.
- Unknown point metrics use the CoT sentinel `999999`.

> **Note:** events submitted as XML store the original `<event>…</event>` element
> in `raw_xml`, so XML responses (`?format=xml` / `Accept: xml`) are lossless:
> unknown detail children and attributes, attribute casing (`releasableto` vs
> `releasableTo`) and element order all survive. The typed columns are only a
> searchable index. Events submitted as JSON have no `raw_xml`, so their XML
> output is synthesised from the typed fields.

## Project layout

```
.
├── Cargo.toml
├── Dockerfile            # multi-stage build, dependency caching
├── fly.toml              # Fly.io app config + release-command migrations
├── compose.yaml          # local Postgres for development
├── migrations/
│   ├── 0001_init.sql     # users
│   ├── 0002_cot_events.sql  # cot_events + cot_latest view
│   └── 0003_raw_xml.sql  # lossless event element (raw_xml)
├── src
│   ├── main.rs           # CLI: `serve` (default) / `migrate`
│   ├── lib.rs
│   ├── config.rs         # environment configuration
│   ├── db.rs             # pool + migrations
│   ├── state.rs          # shared AppState
│   ├── error.rs          # AppError -> JSON responses
│   ├── auth/             # passwords, JWTs, AuthUser extractor
│   ├── models/
│   │   ├── user.rs
│   │   └── cot/          # CoT domain model + XML wire format
│   └── routes/           # axum handlers + router
├── pi-flyio/             # Pi package: Fly.io MCP server, skills, prompts, reference
└── .pi/                  # project Pi settings (loads ./pi-flyio)
```

## Pi integration (`pi-flyio`)

[`pi-flyio/`](pi-flyio/) is a [Pi](https://pi.dev) package for working on
Fly.io projects. It registers Fly's official MCP server (`fly mcp server`),
confirms destructive operations, and ships skills, prompts and reference docs.

- **MCP:** ~60 `mcp__fly__*` tools (apps, machines, volumes, secrets, IPs,
  certificates, orgs), discovered with `tool_search` by default.
- **Skills:** `fly-deploy`, `fly-machines`, `fly-secrets`, `fly-databases`,
  `fly-troubleshooting`, `fly-mcp`.
- **Prompts:** `/fly-deploy`, `/fly-triage`, `/fly-review`.
- **Commands:** `/fly-context`, `/fly-status`, `/fly-doctor`.

In this repository, `.pi/settings.json` declares `./pi-flyio` as a project package,
so it loads once project trust is granted. Install it elsewhere with
`pi install ./pi-flyio`; see [`pi-flyio/README.md`](pi-flyio/README.md) for
configuration and the security model.

## API

| Method | Path                            | Auth | Description                              |
| ------ | ------------------------------- | ---- | ---------------------------------------- |
| GET    | `/health`                       | —    | Liveness probe                           |
| GET    | `/ready`                        | —    | Readiness probe (pings Postgres)         |
| POST   | `/api/v1/auth/register`         | —    | Create an operator, return a JWT         |
| POST   | `/api/v1/auth/login`            | —    | Exchange credentials for a JWT           |
| GET    | `/api/v1/auth/me`               | ✔    | Current operator                         |
| GET    | `/api/v1/users`                 | admin | Operator directory (admin only)          |
| POST   | `/api/v1/cot/events`            | ✔    | Submit a CoT event (JSON or XML)         |
| GET    | `/api/v1/cot/events`            | ✔    | Event feed, newest first (`?since=&uid=&limit=&format=xml`) |
| GET    | `/api/v1/cot/events/latest`     | ✔    | Newest event per `uid` (current picture) |
| GET    | `/api/v1/cot/events/{id}`       | ✔    | A single event (`?format=xml`)           |

Authenticated requests send `Authorization: Bearer <token>`.

- **Write:** CoT XML when `Content-Type: application/xml`, otherwise JSON. The
  response mirrors the request format.
- **Read:** JSON by default; `?format=xml` or `Accept: application/xml` returns
  CoT XML.

### Example — JSON

```bash
curl -sS -X POST http://localhost:8080/api/v1/cot/events \
  -H "authorization: Bearer $TOKEN" -H 'content-type: application/json' \
  -d '{
    "version": 2.0,
    "type": "a-f-G-U-C",
    "access": "Unclassified",
    "uid": "ANDROID-1234",
    "time": "2026-10-21T15:49:23Z",
    "start": "2026-10-21T15:49:23Z",
    "stale": "2026-10-21T15:54:23Z",
    "how": "m-g",
    "point": { "lat": 34.05, "lon": -118.25, "hae": 120.0, "ce": 5.0, "le": 8.0 },
    "detail": {
      "contact": { "callsign": "Ranger-1", "endpoint": "*:-1:stcp" },
      "__group": { "name": "Cyan", "role": "Team Member" },
      "status": { "battery": 87 },
      "track": { "speed": 0.5, "course": 270.0 },
      "remarks": "on station"
    }
  }'
```

### Example — CoT XML

```bash
curl -sS -X POST http://localhost:8080/api/v1/cot/events \
  -H "authorization: Bearer $TOKEN" -H 'content-type: application/xml' \
  --data-binary '<?xml version="1.0" encoding="UTF-8"?>
<event version="2.0" type="a-f-G-U-C" access="Unclassified" uid="ANDROID-1234"
       time="2026-10-21T15:49:23Z" start="2026-10-21T15:49:23Z"
       stale="2026-10-21T15:54:23Z" how="m-g">
  <point lat="34.05" lon="-118.25" hae="999999" ce="5" le="5"/>
  <detail>
    <contact callsign="Ranger-1" endpoint="*:-1:stcp"/>
    <__group name="Cyan" role="Team Member"/>
    <remarks>on station</remarks>
  </detail>
</event>'

# Read the current picture back as CoT:
curl -sS "http://localhost:8080/api/v1/cot/events/latest?format=xml" \
  -H "authorization: Bearer $TOKEN"
```

## Local development

### 1. Get a Supabase connection string

In the Supabase dashboard open **Project Settings → Database → Connection string**
and copy the **Supavisor session pooler** URI (port `5432`):

```
postgresql://postgres.<project-ref>:<password>@aws-0-<region>.pooler.supabase.com:5432/postgres
```

> **Pooler choice matters.** For a long-running server use the **session** pooler
> (port `5432`). If you prefer transaction pooling (port `6543`), set
> `DB_DISABLE_PREPARED_STATEMENTS=true` — Supavisor cannot keep server-side
> prepared statements across transactions.

Prefer a local database? Use the included compose file instead:

```bash
podman compose up -d        # or: docker compose up -d
# DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/ferristak
```

### 2. Configure

```bash
cp .env.example .env
# edit DATABASE_URL, then generate a signing key:
openssl rand -base64 48   # -> JWT_SECRET
```

### 3. Migrate and run

```bash
cargo run -- migrate      # applies 0001 + 0002 (idempotent)
cargo run -- serve        # starts on 0.0.0.0:8080
```

### 4. Test

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Deploying to Fly.io

### 1. Create the app

```bash
fly launch --no-deploy --copy-config   # or: fly apps create ferristak && fly deploy
```

Update the `app` name in `fly.toml` if `ferristak` is taken.

### 2. Set secrets

```bash
fly secrets set \
  DATABASE_URL='postgresql://postgres.<ref>:<password>@aws-0-<region>.pooler.supabase.com:5432/postgres' \
  JWT_SECRET="$(openssl rand -base64 48)"
```

### 3. Deploy

```bash
fly deploy
```

`fly.toml` runs `ferristak migrate` as the
[`release_command`](https://fly.io/docs/reference/configuration/#the-deploy-section)
before rolling out machines, so schema changes are applied exactly once per
deploy. Machines are health-checked at `/health` and scale to zero when idle.

### 4. Operating notes

- **Networking.** The Supavisor pooler is reachable over IPv4 (simplest for Fly
  egress). The direct `db.<ref>.supabase.co` host is IPv6-only.
- **Pool sizing.** Supabase's pooler has a limited number of client slots. Keep
  `DB_MAX_CONNECTIONS` small (5 is a good default).
- **Migrations on boot.** `RUN_MIGRATIONS` defaults to `true` locally but is
  `false` in `fly.toml`, since the release command handles them.

## Configuration reference

| Variable                         | Default        | Notes                                             |
| -------------------------------- | -------------- | ------------------------------------------------- |
| `DATABASE_URL`                   | —              | **Required.** Supabase/Postgres connection string  |
| `JWT_SECRET`                     | —              | **Required.** ≥ 32 bytes                           |
| `JWT_TTL_SECS`                   | `86400`        | Issued-token lifetime                              |
| `HOST` / `PORT`                  | `0.0.0.0` / `8080` | Listener; `BIND_ADDR` overrides both            |
| `BIND_ADDR`                      | —              | Explicit `ip:port` (takes precedence)              |
| `DB_MAX_CONNECTIONS`             | `5`            | Pool size                                          |
| `DB_DISABLE_PREPARED_STATEMENTS` | `false`        | Set `true` for Supavisor transaction mode (6543)   |
| `RUN_MIGRATIONS`                 | `true`         | Apply migrations on startup                        |
| `EVENT_VISIBILITY`               | `owner`        | `owner` = only your events; `shared` = whole team  |
| `CORS_ALLOWED_ORIGINS`           | *(empty)*      | Comma-separated origins; empty = permissive (dev)  |
| `RUST_LOG`                       | `info`         | `tracing` filter directives                        |

## Security notes

- Passwords are hashed with **Argon2id** and a per-hash random salt; hashes are
  never serialized.
- JWTs are validated for signature, algorithm and expiry. Any verification
  failure returns an indistinguishable `401`.
- The token subject is **revalidated against the live account on every
  request**, so a deleted account or a changed role takes effect immediately
  instead of when the token expires. The role in the token is ignored.
- CoT reads are authorization-scoped by `EVENT_VISIBILITY`: `owner` (default)
  limits `list`, `latest` and `{id}` lookups to the caller's own events, while
  `shared` exposes the team picture. Admins always see everything.
- The operator directory is admin-only, so ordinary operators cannot enumerate
  account emails.
- Login and bad-token paths share the same response, avoiding account
  enumeration; emails are stored canonically (lower-cased).
- Internal errors are logged server-side and surfaced as a generic `500`;
  unique/foreign-key violations map to `409`/`422`.
- Requests are body-size limited (256 KiB), and CoT inputs are validated against
  the schema before they reach the database.
- Always set `CORS_ALLOWED_ORIGINS` in production.

## License

MIT — see [LICENSE](LICENSE). The reference schemas under `docs/` retain their
upstream MITRE/TAK licences.
