# flyctl cheatsheet

Curated by task. Run `fly <command> --help` for the full flag set; the CLI is the
source of truth. Everything here also exists as an MCP tool (see
`fly-mcp-tools.md`).

## Identity and environment

```bash
fly version
fly auth whoami
fly auth login
fly auth token
fly orgs list
fly doctor                 # diagnose local networking/auth problems
fly platform status        # is the platform healthy?
```

## Apps

```bash
fly apps list
fly apps create <name>
fly apps destroy <name>    # irreversible: machines and volumes go too
fly apps move <name> --org <org>
fly status                 # machines + health for the app in fly.toml
fly releases               # deploy history (type, time, success, user)
fly restart
```

## Deploy

```bash
fly launch                 # first-time: generates fly.toml + Dockerfile
fly deploy                 # build and roll out
fly deploy --local-only    # build locally, push image
fly deploy --remote-only   # build on Fly's builders
fly config validate        # check fly.toml before deploying
fly config show
fly image show
fly image update
```

## Machines

```bash
fly machine list
fly machine status <id>
fly machine start|stop|restart|suspend|kill <id>
fly machine destroy <id>
fly machine clone <id> --region <region>
fly machine update <id> --vm-memory 512 --vm-cpus 2
fly machine run <image> --region <region>
fly machine exec <id> -- <cmd>
fly logs
fly console                # throwaway machine with the app image + env
fly ssh console            # shell into a running machine
fly ssh console -s         # choose a machine
fly proxy 8080:8080        # tunnel local port to the app
```

## Scale

```bash
fly scale show
fly scale count 3
fly scale count 2 --region sin
fly scale vm shared-cpu-2x
fly scale memory 1024
fly autoscale show|set|disable
```

## Secrets and config

```bash
fly secrets list                     # names + digests, never values
fly secrets set KEY=value [KEY2=value2]
fly secrets set KEY=value --stage    # stage, then `fly secrets deploy`
fly secrets import < .env.production
fly secrets unset KEY
fly secrets deploy
```

## Volumes

```bash
fly volumes list
fly volumes create <name> --region <region> --size <gb>
fly volumes show <id>
fly volumes extend <id> --size <gb>
fly volumes destroy <id>
fly volumes snapshots list <id>
fly volumes snapshots create <id>
```

## Networking, IPs, certificates

```bash
fly ips list
fly ips allocate-v4          # dedicated IPv4 costs money
fly ips allocate-v6
fly ips release <ip>
fly certs list
fly certs add <hostname>
fly certs check <hostname>
fly certs remove <hostname>
```

## Managed Postgres

```bash
fly mpg create --name <db> --region <region>
fly mpg list
fly mpg status <id>
fly mpg connect <id>
fly mpg proxy <id>
fly mpg backups list <id>
```

## MCP

```bash
fly mcp server               # stdio MCP server (what Pi registers)
fly mcp list
fly mcp inspect --url <url>
```

## Machine-readable output

Most commands accept `--json` for scripting:

```bash
fly apps list --json | jq -r '.[].Name'
fly machine list --json | jq '.[] | {id, state, region}'
fly status --json
```
