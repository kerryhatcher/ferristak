# ATAK / TAK server research

Research date: **2026-10-21**. Purpose: establish a traceable public technical basis for designing a Rust server compatible with TAK clients. This is a research and design deliverable; no server functionality or client compatibility has been implemented or demonstrated here.

## Reading map

1. [System scope and conclusions](01-system-overview.md)
2. [Wire protocols, framing, and negotiation](02-wire-protocols.md)
3. [CoT data model and application messages](03-cot-data-model.md)
4. [HTTP APIs, files, missions, and profiles](04-http-interfaces.md)
5. [Identity, enrollment, groups, and security](05-identity-and-security.md)
6. [Federation and other integration interfaces](06-federation-and-integrations.md)
7. [Proposed Rust architecture](07-rust-server-design.md)
8. [Implementation roadmap and compatibility evidence needed](08-roadmap-and-open-questions.md)
9. [Source register and research limitations](09-source-register.md)
10. [Complete archived OpenAPI route inventory](10-api-route-inventory.md)
11. [Schema inventory and differences](11-schema-inventory.md)
12. [PDF catalog](pdfs/README.md)

## Main conclusions

A credible replacement needs separate messaging, identity, and HTTP compatibility surfaces. Relaying CoT XML is the first useful milestone; it does not establish compatibility with file sharing, enrollment, missions, WebTAK, or server federation. The highest risk is silent loss or misrouting of unfamiliar detail fields during XML/protobuf translation. Preserve opaque data and enforce authorization across every delivery and retrieval path.

The public client and server snapshots differ: the client contains registered detail-extension support missing from the archived server detail schema. Generate each schema family separately and select a documented baseline. The saved server guide identifies itself as version 5.7; repository version-history files and generated API metadata are not reliable substitutes for a release identifier.

## Evidence labels

- **Documented:** stated in an archived protocol/schema or official guide.
- **Observed in source:** found in a named function/controller at the pinned public commit; not a promise for every release.
- **Proposed:** engineering recommendation for Ferristak.
- **Unresolved:** requires a target client/server version, additional source tracing, or interoperability captures.

All upstream files are preserved with their notices. [manifest.json](sources/manifest.json) records URLs, commit IDs where applicable, byte counts, and SHA-256 digests. See the [source archive guide](sources/README.md). Public evidence does not establish parity with restricted ATAK military features or certification against the controlled military CoT standard.
