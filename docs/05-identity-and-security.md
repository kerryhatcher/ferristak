# Identity, enrollment, groups, and security

## Separate identities

Maintain distinct certificate identity (issuer/serial/fingerprint), authenticated account, device/contact UID, mutable callsign, connection ID, server group/channel membership, and client display team/role. None is a safe replacement for all the others. Certificate trust establishes possession of a trusted credential; policy still determines account privileges and what traffic may be sent or retrieved. Sources: [group model][group-model], [client auth][stream-impl], [server grouping][submission].

## TLS and certificate provisioning

The official example uses TLS 1.2 and separate client and federation trust stores. It includes 8443 for web/API, 8446 with clientAuth=false for enrollment, and 8089 for messaging. These are configuration examples. A Rust deployment can use PEM/DER internally; it need not adopt Java JKS stores to communicate with clients. Client import/export workflows may still need PKCS#12 containers. Sources: [CoreConfig][config], [configuration guide, security and certificate sections][guide].

**Proposed:** validate certificate chain, validity, client-auth usage, and revocation policy before accepting a session. Bind the validated certificate to an account and permitted device identity. Use separate enrollment and administrative routes/listeners. Enforce hostname validation for outbound connections and client-reachable HTTPS URLs for file services. Support TLS 1.2 for the selected older clients and TLS 1.3 where interoperable. Library capability alone does not establish a certified cryptographic deployment.

## Enrollment protocol

Common Commo generates and retains the client private key locally. It requests `/Marti/api/tls/config`, extracts nameEntries/nameEntry name/value pairs from certificateConfig, includes CN derived from the username, creates a CSR, and strips PEM CSR banners. It POSTs base64 CSR text to `/Marti/api/tls/signClient/v2?clientUid=...`, optionally with version. The upload Content-Type is application/octet-stream; the client requests XML. Sources: [EnrollmentRequest::createURLRequest, parseCsrDoc, parseSigningXML][enroll-client].

Common Commo also distinguishes enrollment credentials: `curlExtraConfig` configures username/password for the ordinary path or an Authorization: Bearer header when useTokenAuth is set. Token validation/issuance is a separate server policy/interface to trace if token enrollment is selected. Do not infer that an enrollment endpoint with clientAuth=false is anonymous. Source: [enrollment HTTP configuration][enroll-client].

Server behavior:

| Endpoint | Body/result |
|---|---|
| GET tls/config | XML certificateConfig, marshaled configuration |
| POST tls/signClient | Base64 CSR input; application/octet-stream PKCS#12 certificate entries signedCert and ca0... |
| POST tls/signClient/v2 + Accept: application/xml | enrollment XML containing signedCert and repeated ca elements, with certificate bodies lacking PEM banners |
| POST tls/signClient/v2 + missing/wildcard/JSON Accept | JSON signedCert and numbered ca entries |

Unsupported Accept values receive bad request in the inspected v2 controller. Source: [CertManagerApi][cert-api]. Legacy signing returns certificate material, not the client's generated private key. The client combines the signed certificate/CA chain with its own private key to produce local stores.

**Proposed:** use authenticated enrollment policy, validate CSR signature and key parameters, derive authorized subject identities on the server, constrain certificate lifetime/usage, record issuance and revocation, and keep signing keys isolated from the message router. Never let arbitrary CSR subject requests mint privileged identities. External CA integration can be a separate adapter. The official guide includes TAKServer and MicrosoftCA signing configurations; reproducing every backend is unnecessary for initial client compatibility. Sources: [configuration example][config], [certificate API][cert-api].

## Group direction and channel selection

TAK groups have IN and OUT directions. Source code tags submitted events with sender IN groups; receiver subscription state tracks OUT groups. The direction is from the server's perspective: IN is publishing into the server, OUT is receiving from it. The basic proposed authorization expression is an intersection of event ingress groups and recipient egress groups, supplemented with explicit destination/mission constraints. Sources: [submission group assignment][submission], [MessagingUtilImpl][messaging], [group model][group-model].

The guide's introductory shared-group explanation is simplified. Input filtergroup settings can label all traffic arriving on a configured input. File/LDAP authentication messages or certificate identity can provide user memberships. `__ANON__` is the literal anonymous group name; it must not silently become the production fallback for authenticated failures. Sources: [guide §§9.3–9.6][guide], [input configuration][config].

The active-group API lets clients select permitted channels; selection must be a subset of authenticated entitlements. A client-supplied Group array, group bit vector, display team, file Groups parameter, or marti destination is not independently authoritative. Apply the same policy to live delivery, latest-state replay, contact lookup, history, file search/download, mission subscriptions and federation.

## Concrete failure modes to design out

| Failure | Required design response (proposed) |
|---|---|
| Cross-group contact/file/history leakage | Shared authorization service on reads and sends; scope every query |
| Credential XML recorded as event | Separate root decoder and redaction before any generic logging |
| Unbounded partial frame or ZIP expansion | Byte/depth/time/entry quotas before allocation/extraction |
| Recipient callsign collision or UID impersonation | Account/device binding and explicit collision policy |
| Permission revocation while output is queued | Policy generation tags; recheck or invalidate queued delivery |
| Event markings lost during conversion | Preserve access/caveat/releasableTo and opaque data |
| Shared content hash exposes another tenant's asset | Distinct authorization metadata independent of dedup bytes |
| Slow client exhausts router memory | Bounded per-session queues and documented disconnect/drop rules |
| Unknown extension silently discarded | Opaque preservation or explicit incompatibility handling |

These are design requirements inferred from the interface structure, not vulnerability claims about the official server. Marking preservation does not amount to cross-domain security enforcement or military accreditation.


<!-- Archived primary sources; exact revisions are recorded in sources/manifest.json. -->
[stream-impl]: sources/atak-civ/commoncommo/core/impl/streamingsocketmanagement.cpp
[enroll-client]: sources/atak-civ/commoncommo/core/impl/enrollmentmanager.cpp
[config]: sources/Server/src/takserver-core/example/CoreConfig.example.xml
[cert-api]: sources/Server/src/takserver-core/takserver-war/src/main/java/com/bbn/tak/tls/CertManagerApi.java
[group-model]: sources/Server/src/takserver-common/src/main/java/com/bbn/marti/remote/groups/Group.java
[messaging]: sources/Server/src/takserver-core/src/main/java/com/bbn/marti/groups/MessagingUtilImpl.java
[submission]: sources/Server/src/takserver-core/src/main/java/com/bbn/marti/service/SubmissionService.java
[guide]: pdfs/TAK_Server_Configuration_Guide.pdf
