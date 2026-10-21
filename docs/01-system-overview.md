# System scope and design conclusions

## What is being replaced

ATAK is the Android client in the Team Awareness Kit ecosystem. TAK Server supplies shared communication and services for clients; a Rust server alternative is a replacement for those server interfaces. The public ATAK-CIV implementation exposes the communications machinery needed to research interoperability. Its published code cannot establish the complete interfaces of restricted military builds or every plugin.

A CoT event describes an entity or occurrence in space and time. TAK adds persistent streams, protobuf encoding, routing conventions, file services, mission synchronization, certificate enrollment, and federation. The historical CoT guide describes one-event TCP transfers; the later TAK protocol explicitly adds persistent streaming. Treat these as different layers with different contracts. Sources: [CoT guide][cot-guide], [TAK wire protocol][wire], [server configuration][config].

## Service boundaries

| Surface | Role | Primary evidence | Initial priority |
|---|---|---|---|
| CoT message transport | Position, map objects, chat, control events | Protocol README, Common Commo | Essential |
| Identity and routing | Certificate validation, groups, directed recipients | Enrollment and messaging code | Essential |
| Marti HTTP compatibility | Discovery, contacts, packages, content | OpenAPI plus servlet implementations | Essential after messaging |
| Data Sync | Persistent shared missions, subscriptions, content changes | MissionApi and schemas | Separate milestone |
| Device profiles | Connection/enrollment/tool configuration packages | Profile controllers and OpenAPI | Deployment dependent |
| Federation | Server identity, groups, events, content, provenance | fig.proto and official guide | Later milestone |
| Browser and plugin interfaces | Web clients and server extension ecosystem | Additional version-specific interfaces | Explicitly select scope |

Sources: [configuration guide][guide], [API snapshot][openapi], [mission controller][missions], [federation schema][fig].

## Listener map

These are examples from the pinned official CoreConfig, not mandatory protocol port numbers. A deployed configuration can change them.

| Port / network | Example use | Notes |
|---|---|---|
| TCP 8089 | TLS bidirectional client messaging | XML first, optional negotiated protobuf |
| UDP 8090 | QUIC client input in current example | Separate transport; public client has QUIC code |
| TCP 8443 | HTTPS Marti/API and web connector | Normally client certificate authenticated |
| TCP 8446 | Enrollment HTTPS connector | Example sets clientAuth=false; application authentication still matters |
| TCP 8444 | HTTPS using federation trust store | Distinct from federation event transport |
| TCP 9000 | Legacy federation | Example has v1 disabled |
| TCP 9001 | Federation v2 | Example has v2 enabled |
| TCP 8088 | Commented stcp input example | Plain persistent streaming |
| TCP/UDP 8087 | Commented TCP/UDP input examples | Verify mode; do not infer from port alone |
| UDP 239.2.3.1:6969 | Commented situational-awareness multicast input | LAN integration, not WAN server discovery |
| UDP 224.10.10.1:17012 | Commented GeoChat multicast input | Deployment dependent |

Source: [CoreConfig example][config]. Do not hard-code common community port assumptions into parsers. Raw TAK TLS messaging is not HTTPS; an HTTP reverse proxy cannot terminate it as an HTTP route.

## Compatibility target

**Proposed:** begin with a named ATAK-CIV release using pre-provisioned client certificates, CoT XML and protobuf v1, group-limited routing, directed chat, and server-backed packages. Add automated enrollment and missions as explicit later acceptance gates. Record WinTAK and iTAK compatibility independently.

**Unresolved:** exact target application versions, plugin requirements, required mission semantics, required federation peers, expected offline behavior, and operational capacity. Neither a port opening nor a successful login demonstrates application interoperability.


<!-- Archived primary sources; exact revisions are recorded in sources/manifest.json. -->
[wire]: sources/atak-civ/takproto/README.txt
[config]: sources/Server/src/takserver-core/example/CoreConfig.example.xml
[openapi]: sources/Server/src/takserver-takcl-core/src/main/openapi/takserver-openapi-spec.json
[missions]: sources/Server/src/takserver-core/takserver-war/src/main/java/com/bbn/marti/sync/api/MissionApi.java
[fig]: sources/Server/src/takserver-protobuf/src/main/proto/fig.proto
[guide]: pdfs/TAK_Server_Configuration_Guide.pdf
[cot-guide]: pdfs/CoT_Developers_Guide_2005.pdf
