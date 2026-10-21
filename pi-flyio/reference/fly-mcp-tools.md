# Fly.io MCP tool catalog

> Generated from the live `fly mcp server` (flyctl v0.4.120 linux/amd64 Commit: b3263736a487b5c713fc7733633cfbb2e2976dd6 BuildDate: 2026-10-20T12:27:01Z). Do not edit by hand;
> regenerate with `python3 scripts/gen-fly-mcp-catalog.py`.

The server exposes **60 tools**. Pi registers it under the namespace
`mcp__fly`, so a tool appears as `mcp__fly__<name>` (characters other than letters,
digits and `_` become `_`).

`Access` is this package's policy, not the server's. The Fly server marks every tool
`destructiveHint: true`, so `pi-flyio` allow-lists reads and confirms everything else.
Every tool also counts as `openWorldHint: true`.

| MCP tool | Access | Description |
|---|---|---|
| `mcp__fly__fly_apps_create` | confirmed | Create a new Fly.io app.  If you don't specify a name, one will be generated for you. |
| `mcp__fly__fly_apps_destroy` | confirmed | Destroy a Fly.io app.  All machines and volumes will be destroyed. |
| `mcp__fly__fly_apps_list` | read-only | List all Fly.io apps in the organization |
| `mcp__fly__fly_apps_move` | confirmed | Move a Fly.io app to a different organization |
| `mcp__fly__fly_apps_releases` | read-only | List all releases for a Fly.io app, including type, when, success/fail and which user triggered the release. |
| `mcp__fly__fly_apps_restart` | confirmed | Restart a Fly.io app. Perform a rolling restart against all running Machines. |
| `mcp__fly__fly_certs_add` | confirmed | Add an SSL/TLS certificate for a Fly.io app. |
| `mcp__fly__fly_certs_check` | read-only | Check the status of an SSL/TLS certificate for a Fly.io app. |
| `mcp__fly__fly_certs_list` | read-only | List all SSL/TLS certificates for a Fly.io app. |
| `mcp__fly__fly_certs_remove` | confirmed | Remove an SSL/TLS certificate for a Fly.io app. |
| `mcp__fly__fly_certs_show` | read-only | Show details for an SSL/TLS certificate for a Fly.io app. |
| `mcp__fly__fly_ips_allocate_v4` | confirmed | Allocate an IPv4 address to the application. Dedicated IPv4 addresses cost $2/mo. |
| `mcp__fly__fly_ips_allocate_v6` | confirmed | Allocate an IPv6 address to the application |
| `mcp__fly__fly_ips_list` | read-only | List all IP addresses allocated to the application |
| `mcp__fly__fly_ips_private` | confirmed | List instances' private IP addresses, accessible from within the Fly network |
| `mcp__fly__fly_ips_release` | confirmed | Release one or more IP addresses from the application |
| `mcp__fly__fly_logs` | read-only | Get logs for a Fly.io app or specific machine |
| `mcp__fly__fly_machine_clone` | confirmed | Clone a Fly Machine. The new Machine will be a copy of the specified Machine. If the original Machine has a volume, then a new empty volume will be created and attached to the new Machine. |
| `mcp__fly__fly_machine_cordon` | confirmed | Deactivate all services on a machine |
| `mcp__fly__fly_machine_create` | confirmed | Create, but don’t start, a machine |
| `mcp__fly__fly_machine_destroy` | confirmed | Destroy one or more Fly machines. This command requires a machine to be in a stopped or suspended state unless the force flag is used. |
| `mcp__fly__fly_machine_egress_ip_allocate` | confirmed | Allocate a pair of static egress IPv4 and IPv6 for a machine |
| `mcp__fly__fly_machine_egress_ip_list` | read-only | List all static egress IPv4 and IPv6 for a machine |
| `mcp__fly__fly_machine_egress_ip_release` | confirmed | Release a pair of static egress IPv4 and IPv6 for a machine |
| `mcp__fly__fly_machine_exec` | confirmed | Run a command on a machine |
| `mcp__fly__fly_machine_kill` | confirmed | Kill (SIGKILL) a Fly machine |
| `mcp__fly__fly_machine_leases_clear` | confirmed | Clear the leases for a machine |
| `mcp__fly__fly_machine_leases_view` | read-only | View machine leases |
| `mcp__fly__fly_machine_list` | read-only | List all machines for a Fly app |
| `mcp__fly__fly_machine_restart` | confirmed | Restart a Fly machine |
| `mcp__fly__fly_machine_run` | confirmed | Run a machine |
| `mcp__fly__fly_machine_start` | confirmed | Start a Fly machine |
| `mcp__fly__fly_machine_status` | read-only | Show current status of a running machine |
| `mcp__fly__fly_machine_stop` | confirmed | Stop a Fly machine |
| `mcp__fly__fly_machine_suspend` | confirmed | Suspend a Fly machine |
| `mcp__fly__fly_machine_upcordon` | confirmed | Reactivate all services on a machine |
| `mcp__fly__fly_machine_update` | confirmed | Update a machine |
| `mcp__fly__fly_orgs_create` | confirmed | Create a new organization. Other users can be invited to join the organization later. |
| `mcp__fly__fly_orgs_delete` | confirmed | Delete an organization. All apps and machines in the organization will be deleted. |
| `mcp__fly__fly_orgs_invite` | confirmed | Invite a user, by email, to join organization. The invitation will be sent, and the user will be pending until they respond. |
| `mcp__fly__fly_orgs_list` | read-only | List all organizations the user is a member of.  Keys are names of the organizations, values are slugs. |
| `mcp__fly__fly_orgs_remove` | confirmed | Remove a user from an organization. The user will no longer have access to the organization. |
| `mcp__fly__fly_orgs_show` | read-only | Shows information about an organization. Includes name, slug and type. Summarizes user permissions, DNS zones and associated member. Details full list of members and roles. |
| `mcp__fly__fly_platform_regions` | read-only | View a list of regions where Fly has datacenters. |
| `mcp__fly__fly_platform_status` | read-only | Get the status of Fly's platform |
| `mcp__fly__fly_platform_vm_sizes` | read-only | Get a list of VM sizes available for Fly apps |
| `mcp__fly__fly_secrets_deploy` | confirmed | Deploy secrets to the specified app |
| `mcp__fly__fly_secrets_list` | read-only | List secrets for the specified app |
| `mcp__fly__fly_secrets_set` | confirmed | Set secrets for the specified app; secrets are staged for the next deploy |
| `mcp__fly__fly_secrets_unset` | confirmed | Unset secrets for the specified app; changes are staged for the next deploy |
| `mcp__fly__fly_status` | read-only | Get status of a Fly.io app |
| `mcp__fly__fly_volumes_create` | confirmed | Create a new volume for an app. Volumes are persistent storage for Fly Machines. |
| `mcp__fly__fly_volumes_destroy` | confirmed | Destroy one or more volumes. When you destroy a volume, you permanently delete all its data. |
| `mcp__fly__fly_volumes_extend` | confirmed | Extend a volume to a larger size. You can only extend a volume to a larger size. |
| `mcp__fly__fly_volumes_fork` | confirmed | Fork the specified volume. Volume forking creates an independent copy of a storage volume for backup, testing, and experimentation without altering the original data. |
| `mcp__fly__fly_volumes_list` | read-only | List all volumes for an app. Volumes are persistent storage for Fly Machines. |
| `mcp__fly__fly_volumes_show` | read-only | Show details about a volume. Volumes are persistent storage for Fly Machines. |
| `mcp__fly__fly_volumes_snapshots_create` | confirmed | Create a snapshot of a volume. Snapshots are point-in-time copies of a volume. |
| `mcp__fly__fly_volumes_snapshots_list` | read-only | List all snapshots for a volume. Snapshots are point-in-time copies of a volume. |
| `mcp__fly__fly_volumes_update` | confirmed | Update a volume. You can activate or deactivate snapshotting, and change the snapshot's retention period. |

