# Federation and integration interfaces

## Federation is a separate protocol family

The official example configures legacy federation on 9000 disabled, federation v2 on 9001 enabled, and a separate 8444 federation-trust HTTPS connector. Its federation TLS store is distinct from the normal client trust store. These are deployment defaults/examples, not fixed wire requirements. Sources: [CoreConfig][config], [guide §12][guide].

The public `fig.proto` defines gRPC service `com.atakmap.FederatedChannel`. Its `FederatedEvent` contains GeoEvent, contact updates, federateGroups, provenance, hop counts, and group hop limits. GeoEvent is **not** the client streaming CotEvent: field tags and structure differ. Use separately generated types. Source: [federation schema][fig].

## Observed gRPC contract

| Method | RPC shape / role |
|---|---|
| SendOneEvent | Unary FederatedEvent submission |
| BinaryMessageStream | Client stream of BinaryBlob |
| SendOneBlob | Unary binary submission |
| getIdentity | Unary server identity |
| ClientEventStream | Subscription request, server event stream |
| ServerEventStream | Client event stream, subscription result |
| HealthCheck | Client/server health exchange |
| ClientROLStream / ServerROLStream | Server/client streams for ROL payloads |
| ServerFederateGroupsStream | Subscription request, server group stream |
| ClientFederateGroupsStream | Client group stream |
| GetAuthTokenByX509 | Certificate-related token exchange |
| Getx509Identity | Certificate identity retrieval |

Preserve method casing exactly. The paired event methods are not a single bidirectional RPC. Identity includes name, UID, description, connection type and serverId; subscriptions also carry a filter and server version. Source: [fig.proto][fig].

## Semantics beyond protobuf generation

Implementing the schema does not establish federation interoperability. Required behavior includes peer trust and identity, incoming/outgoing group mappings, provenance and hop-limit handling, duplicate loop prevention, contact changes, reconnects, subscriptions, file/mission sharing and disruption recovery. The guide covers federation group mapping, mission disruption tolerance, and file blocking. Sources: [federation guide][guide], [schema][fig].

**Proposed:** defer federation until local routing/authorization is stable. If added, isolate it as an authenticated adapter that produces the same internal events and policy context as client ingress. Retain upstream provenance, apply explicit local group mapping, enforce hops on every relay and avoid falling back to anonymous delivery. Define replay/dedup keys independently of CoT entity UID, since repeated reports legitimately share a UID.

**Unresolved:** exact legacy federation framing and message rules were not fully traced; v2 handshake sequencing, token use, mandatory RPC combinations, content retrieval and mission recovery require peer captures and more implementation tracing. Do not advertise federation parity based only on a successful HealthCheck.

## Other public integration surfaces

- `streaminginput.proto` defines `com.atakmap.InputChannel`: ClientTakMessageStream, ServerTakMessageStream and VersionCheck. It is a distinct gRPC input interface and does not replace the 0xBF-framed ATAK stream. Source: [streaming input schema][grpc-input].
- The pinned client and CoreConfig include QUIC support. It needs its own wire/identity/state analysis; this research does not infer that QUIC is simply a TCP codec wrapped in UDP. Source: [current configuration][config], [client transport manager][stream-impl].
- Local multicast bridges must implement mesh framing/version selection separately from server-stream negotiation. Source: [wire spec][wire].
- ATAK plugins often communicate by extending detail XML. Preserve unknown subtrees even when no matching server plugin exists. Native ATAK plugin APIs and TAK Server's Java plugin APIs are separate from the network contract. Source: [detail mapping][detail-proto].
- WebTAK/browser compatibility requires version-specific WebSocket, authentication and HTTP behavior. Browser support was not fully traced and is outside the initial profile. Server plugin binary compatibility is also outside that profile.
- Map layers and video catalogs distribute descriptions/references. CoT message routing alone is not an RTSP/video streaming server. Select media service scope independently.


<!-- Archived primary sources; exact revisions are recorded in sources/manifest.json. -->
[wire]: sources/atak-civ/takproto/README.txt
[detail-proto]: sources/atak-civ/takproto/detail.proto
[stream-impl]: sources/atak-civ/commoncommo/core/impl/streamingsocketmanagement.cpp
[config]: sources/Server/src/takserver-core/example/CoreConfig.example.xml
[fig]: sources/Server/src/takserver-protobuf/src/main/proto/fig.proto
[grpc-input]: sources/Server/src/takserver-protobuf/src/main/proto/streaminginput.proto
[guide]: pdfs/TAK_Server_Configuration_Guide.pdf
