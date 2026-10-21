# HTTP interfaces, packages, missions, and profiles

## Interface evidence and caveats

The archived [OpenAPI 3.0.1 document][openapi] has 290 paths. Its generic info version `v0` is not the TAK Server release. Its generated server URL `https://127.0.0.1:17655` is a generation environment value, not a required deployment address. The [route inventory](10-api-route-inventory.md) lists every documented method with parameters and request/response schema references.

OpenAPI is incomplete for interoperability: legacy `/Marti/sync/*` servlets implement actual package transfer and are not interchangeable with `/Marti/api/sync/*`. It also misleads on some content types: signing is represented as an application/json string body, whereas Common Commo sends base64 CSR text with application/octet-stream and negotiates XML output. Trace client calls and controllers together. Sources: [enrollment client][enroll-client], [certificate controller][cert-api], [package client][mp-client].

## High-value endpoint map

All paths below include the deployment context and spelling/case that clients expect. Methods and details are based on the pinned source/API snapshot, not executed requests.

| Method and path | Purpose | Key contract |
|---|---|---|
| GET /Marti/api/version | Version discovery | Plain version string |
| GET /Marti/api/version/info | Structured version | VersionInfo schema |
| GET /Marti/api/version/config | Client-facing configuration | API envelope containing version/api/hostname |
| GET /Marti/api/clientEndPoints | Contact discovery | secAgo, showCurrentlyConnectedClients, showMostRecentOnly, group |
| GET /Marti/api/contacts/all | Subscription/contact listing | Sort/direction/noFederates options |
| GET /Marti/api/groups/all | Allowed groups | useCache, sendLatestSA |
| PUT /Marti/api/groups/active | Select active channels | Group array plus optional clientUid |
| PUT /Marti/api/groups/activebits | Select active channel bits | Integer array plus optional clientUid |
| GET /Marti/api/cot/xml/{uid} | Retrieve event XML | UID lookup with authorization |
| GET /Marti/api/cot/xml/{uid}/all | Retrieve event history | secago/start/end |
| GET /Marti/api/sync/search | Resource search | Name/hash/UID, time, spatial, tool and keyword filters |
| PUT /Marti/api/sync/metadata/{hash}/{metadata} | Resource metadata update | Body and accepted metadata values are controller specific |
| GET /Marti/api/tls/config | Enrollment subject configuration | certificateConfig XML |
| POST /Marti/api/tls/signClient | Legacy enrollment | PKCS#12 certificate container response |
| POST /Marti/api/tls/signClient/v2 | New enrollment | XML or JSON signed certificate/CA chain response |

Sources: [OpenAPI][openapi], [version controller][version], [group controller][groups], [subscriptions controller][subscriptions], [certificate controller][cert-api]. Generic API envelopes contain version, type, data, optional messages, and nodeId; many endpoints return strings/XML/binary directly instead. Preserve each endpoint's response shape. Source: [ApiResponse][api-response].

## Server-backed package transfer

Observed Common Commo sequence:

1. GET `/Marti/sync/missionquery?hash=<sha256>` checks whether accessible content already exists. A hit returns a URL string pointing to `/Marti/sync/content?hash=...`; a miss is 404.
2. POST `/Marti/sync/missionupload?hash=...&filename=...&creatorUid=...` sends multipart/form-data with an `assetfile` part. The client uses application/x-zip-compressed for the part. The server also accepts a `resource` part for browser uploads.
3. The server computes the content identifier from uploaded bytes; its comment explicitly says the supplied hash is ignored. A successful upload returns a content URL string, not the usual JSON envelope.
4. PUT `/Marti/api/sync/metadata/<hash>/tool` with text/plain marks the client upload public for server-only sharing or private for contact sends.
5. A CoT fileshare announcement conveys senderUrl, sha256, sizeInBytes, filename, senderUid, senderCallsign, name, and possibly peerHosted/httpsPort. Recipient downloads content over HTTP(S), verifies it, and can acknowledge through CoT.

Sources: [client upload state machine][mp-client], [query servlet][query], [upload servlet][upload], [fileshare event builder][cot-impl].

GET and HEAD `/Marti/sync/content` support hash/UID retrieval; the servlet has offset/length and partial-content handling. Match its actual headers/status/body behavior before claiming resumable transfer support. A hash is an identifier, not an access token: both query and download must enforce group/resource access. Sources: [content servlet][content], [query servlet][query].

**Proposed storage:** stream to a temporary file, calculate SHA-256 server-side, validate size and allowed container metadata, commit immutable bytes, then transactionally publish authorized metadata. Physical deduplication must not share authorization records across tenants. Return a client-reachable configured HTTPS base URL; never reflect an untrusted Host header into download links.

## Package container

TAK packages are ZIP containers with a mission-package manifest. The archived manifest XSD defines MissionPackageManifest, Configuration/Parameter, Contents/Content with zipEntry and ignore, and optional Groups. Contents can reference CoT XML and arbitrary assets. A configuration/enrollment package and a map-content package can use the same packaging machinery but require different trust decisions. Source: [manifest XSD][manifest-xsd].

**Unresolved:** confirm manifest path/casing and parameter conventions using packages generated by each target client; the XSD alone cannot specify ZIP layout. **Proposed:** validate paths, reject traversal/symlinks and duplicate dangerous entries, bound expanded size/count/ratio, and do not execute or automatically apply uploaded preferences. Files can be stored opaquely when server-side extraction is unnecessary.

## Data Sync missions

A persistent mission is not just a ZIP file. The MissionApi exposes mission metadata, membership/subscriptions, role permissions, content references, changes, invitations, logs, and name/GUID addressing. Important route families include:

- GET `/Marti/api/missions`; GET/PUT/POST/DELETE `/Marti/api/missions/{name}`.
- Name and GUID forms for mission content and lookup.
- Mission subscriptions and role changes; use the exact route inventory rather than inventing RESTful spellings.
- Changes and time-window retrieval; content mutation and CoT notification must agree.

Source: [MissionApi][missions], [OpenAPI][openapi]. Creation parameters include creatorUid, group, description, chatRoom, baseLayer, bbox, classification, tool, password, defaultRole, expiration, inviteOnly, and allowGroupChange in the relevant method. Some take arrays or query values rather than a conventional JSON object. Role values include MISSION_OWNER, MISSION_SUBSCRIBER, and MISSION_READONLY_SUBSCRIBER. Do not reconstruct this interface from endpoint names alone.

**Proposed:** transactions record mission change sequence and durable outbox notification together. Reconnecting clients fetch changes and current membership; continuous socket delivery alone cannot provide offline recovery. GUID/name collisions, deletion/tombstones, passwords, and invitations need version-specific captures before implementation.

## Device profiles and secondary services

The API snapshot includes connection, enrollment and tool-specific profile downloads plus profile administration, file/directory management and generated mission packages. Profiles can alter client configuration, so authorization and package trust deserve their own boundary. Video connection catalogs, iconsets, map layers, KML, CoT history, and specialized checklist/report APIs appear in the broader API surface. Implement only those selected for the compatibility profile; inventory presence does not imply mandatory MVP scope. Source: [complete OpenAPI][openapi].


<!-- Archived primary sources; exact revisions are recorded in sources/manifest.json. -->
[cot-impl]: sources/atak-civ/commoncommo/core/impl/cotmessage.cpp
[mp-client]: sources/atak-civ/commoncommo/core/impl/missionpackagemanager.cpp
[enroll-client]: sources/atak-civ/commoncommo/core/impl/enrollmentmanager.cpp
[openapi]: sources/Server/src/takserver-takcl-core/src/main/openapi/takserver-openapi-spec.json
[missions]: sources/Server/src/takserver-core/takserver-war/src/main/java/com/bbn/marti/sync/api/MissionApi.java
[upload]: sources/Server/src/takserver-core/takserver-war/src/main/java/com/bbn/marti/sync/MissionPackageUploadServlet.java
[query]: sources/Server/src/takserver-core/takserver-war/src/main/java/com/bbn/marti/sync/MissionPackageQueryServlet.java
[content]: sources/Server/src/takserver-core/takserver-war/src/main/java/com/bbn/marti/sync/ContentServlet.java
[manifest-xsd]: sources/Server/src/takserver-core/takserver-war/src/main/java/com/bbn/marti/util/missionpackage/mp.xsd
[cert-api]: sources/Server/src/takserver-core/takserver-war/src/main/java/com/bbn/tak/tls/CertManagerApi.java
[groups]: sources/Server/src/takserver-core/takserver-war/src/main/java/com/bbn/marti/groups/GroupsApi.java
[subscriptions]: sources/Server/src/takserver-core/takserver-war/src/main/java/com/bbn/marti/sync/api/SubscriptionApi.java
[api-response]: sources/Server/src/takserver-core/takserver-war/src/main/java/com/bbn/marti/cot/search/model/ApiResponse.java
[version]: sources/Server/src/takserver-core/takserver-war/src/main/java/com/bbn/marti/util/VersionApi.java
