# Roadmap, evidence plan, and unresolved questions

## Implementation gates

These gates are proposed future validation work. No application tests, interoperability exercises, or server benchmarks were run for this documentation task.

| Gate | Deliverable | Acceptance evidence to obtain |
|---|---|---|
| 0 | Named target client/server profile | Version/build IDs, approved sample packages, recorded options |
| 1 | CoT XML router over TLS | Two target clients exchange presence, map item and chat; unauthorized groups isolated |
| 2 | Protobuf v1 negotiation and conversion | XML-only and binary clients share events without detail loss; framing boundary captures |
| 3 | Contacts and packages | Query/upload/download, correct URL strings, receipts, permission-denied behavior |
| 4 | Enrollment and profiles | New device enrolls, retains its private key, reconnects, renewal/revocation exercised |
| 5 | Data Sync | Missions, roles, contents, reconnect/change recovery, deletion and denied writes |
| 6 | Operational hardening | Slow peers, bounded memory, partial frames, storage outage, audit and restart recovery |
| 7 | Additional clients and federation | Independent WinTAK/iTAK results; selected peer version/RPC semantics verified |

## Protocol fixtures to collect

- XML messages split at every byte boundary, adjacent documents, XML declaration variants and authentication followed immediately by events.
- Offer/request/accept, denial/retry, no offer, delayed response, timeout and disconnect; XML acceptance plus binary frame in one read.
- Binary frames with multi-byte lengths, partial/overflow varints, invalid magic, oversize payload, unknown fields and control-only TakMessage.
- Typed details with extra attributes, duplicate children, malformed typed values, arbitrary plugin XML, Unicode and registered unknown extensions.
- Presence expiry/reconnect, multiple devices per account, duplicate callsigns, UID collision, directed destinations and self echo.
- Package upload and hit/miss query responses, multipart field names, download HEAD/partial response, filenames, denial, content hashes and receipts.
- Enrollment config, legacy and v2 responses, Accept behavior, trust-chain rotation, renewal, revocation and incorrect CSR identity.
- Mission subscription, concurrent updates, changes and offline recovery, read-only role, passwords, invitations, expiry and delete.

These are an evidence plan, not an included packet corpus. Prefer synthetic accounts and consented lab captures. Keep credentials/private keys out of the repository.

## Gap register

| Question | Current evidence | Remaining work |
|---|---|---|
| Full military ATAK parity | Public ATAK-CIV source only | Authorized military interface requirements if in scope |
| MIL-STD-6090A conformance | DLA public metadata; controlled full standard | Obtain through authorized channels; no public conformance claim |
| Selected app versions | Not specified | Pin exact target clients before implementing |
| Newer extension interop | Client schema supports field 8; server snapshot differs | Select baseline and collect extension-enabled examples |
| Markings enforcement | Public fields/comments and passthrough rules | Define permitted policy; preservation alone is insufficient |
| All destination precedence | Client constructors and server sources | Trace filters and compare captures; especially mixed destinations |
| Full chat receipts/emergency behavior | XSDs, builders, repeater configuration | Version-specific cancellation/ack/resend fixtures |
| Complete Data Sync contract | Public controllers and generated API | Capture responses/notifications/error and offline behavior |
| Package ZIP conventions | Manifest schema and transfer code | Export from target clients; confirm paths and parameter meanings |
| Legacy federation | Guide/config mentions | Trace and document exact v1 implementation if required |
| Federation v2 session behavior | Complete public proto, high-level guide | Handshake/auth/RPC ordering, loops, reconnect/file recovery |
| QUIC | Current config and client code include it | Dedicated transport/protocol research; deferred |
| WebTAK | HTTP inventory and server repository | Trace WebSocket and browser auth versions; deferred |
| Capacity and resource defaults | Architecture estimates only | Define workload, measure router and storage behavior |

## Decisions before coding

Choose the initial compatibility profile, maximum frame/package sizes, retention and stale handling, sender/device binding, duplicate callsign policy, authentication backend, certificate authority integration and mission/federation scope. Use documented defaults as configurable product decisions rather than presenting them as TAK requirements.

## Research boundaries

The archived official sources are pinned snapshots, not a claim of the latest deployed military software. Generated OpenAPI omits legacy servlets and can misstate request media types. The public historical CoT guide is conceptual and predates TAK streams. The saved configuration guide is operational documentation, not a complete byte-level API specification. None of these limitations prevents an initial Rust design, but each matters to claims of drop-in replacement compatibility.
