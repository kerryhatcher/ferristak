---
description: Diagnose an unhealthy Fly.io app from logs and state
argument-hint: "[app]"
---
Diagnose why ${1:-this Fly app} is unhealthy.

Follow the `fly-troubleshooting` skill. Establish state first: `fly status`, then
`fly logs`, then `fly releases` (prefer the read-only MCP tools where possible).
Then present:

- current machine states and health-check results,
- the most relevant log lines, verbatim, with timestamps,
- the most likely root cause(s), ranked, each with the evidence that supports it,
- the smallest safe next action and what it would prove.

Do not mutate anything until you have reported the diagnosis and I confirm.
