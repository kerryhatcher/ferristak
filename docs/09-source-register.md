# Source register

Retrieved 2026-10-21. [Machine-readable manifest](sources/manifest.json) contains every archived file, its source URL, commit ID where applicable, local path, byte count and SHA-256. Preserve original source and license notices. The original texts are stored as reference material, not incorporated into the Rust application build.

## Primary sources

| Source | Snapshot / location | Use |
|---|---|---|
| [TAK Product Center ATAK-CIV](https://github.com/TAK-Product-Center/atak-civ) | Commit 9f6893dd657feacc35ec5de03dad721c2e44170e | Wire protocol, Common Commo implementation, client schemas and public CoT XSDs |
| [TAK Product Center Server](https://github.com/TAK-Product-Center/Server) | Commit 5187abd46d827d37cfc5708805eced197a837e49 | HTTP controllers/servlets, config, API snapshot, federation and server schemas |
| [TAK protocol README][wire] | Archived raw text | Framing, version selection and protocol negotiation |
| [Client protobuf family][client-proto] | takproto directory | Version 1 field mappings and typed detail rules |
| [Public base event XSD][cot-xsd] | MITRE schema as published in ATAK repository | CoT core fields, type hierarchy, validation and public amendments |
| [TAK Server configuration guide][guide] | PDF identifies version 5.7 | Installation, authentication, filtering, federation and certificates |
| [CoT developer guide][cot-guide] | Public release, dated August 2005 | Original concepts and extension philosophy; historical context |
| [Generated OpenAPI][openapi] | OpenAPI 3.0.1, info version v0 | 290 route paths and component schemas, with omissions described in HTTP research |
| [DLA MIL-STD-6090 metadata](https://quicksearch.dla.mil/qsDocDetails.aspx?ident_number=284928) | Active record reviewed on research date | Confirms standard identity/revision and controlled distribution status |

The pinned server VERSION.md begins with older 4.5.0 history while its PDF identifies 5.7. Do not label all archived sources 4.5 based on that history file or label the API release v0 based on generated metadata.

## Additional implementation references

[FreeTAKTeam/FreeTakServer](https://github.com/FreeTAKTeam/FreeTakServer) and [tkuester/taky](https://github.com/tkuester/taky) are useful independent implementation references. Their public repositories were reviewed at overview level, not audited or used as authoritative definitions of the official protocol. The former identifies Eclipse Public License and the latter MIT in their repository materials. Detailed compatibility claims about these implementations are deliberately not inferred from their descriptions.

Rust primary documentation reviewed for architecture candidates: [prost](https://docs.rs/prost/latest/prost/), [quick-xml](https://docs.rs/quick-xml/latest/quick_xml/), [Tokio broadcast](https://docs.rs/tokio/latest/tokio/sync/broadcast/index.html), [rustls](https://docs.rs/rustls/latest/rustls/). The links are moving latest documentation, so revisit and pin exact crate versions during implementation.

## Important unavailable documents

DLA lists MIL-STD-6090 Revision A dated 2025-01-31, 508 pages, Distribution C, and the 2022 base document as Distribution C. The controlled full text was not downloaded or used. The publicly released CoT XSD and historical guide are saved; they do not substitute for authorized access when a contract requires conformance to the current military standard.

MITRE's [CoT Message Router User's Guide landing page](https://www.mitre.org/news-insights/publication/cursor-target-message-router-users-guide-0) links [09_4937.pdf](https://www.mitre.org/sites/default/files/pdf/09_4937.pdf). The browser research tool could read it; direct download returned HTTP 403. It is listed in the PDF catalog as **not saved**, and this research does not depend on it for modern TAK wire behavior.

## Evidence and reproduction

The manifest uses pinned raw GitHub URLs, so reference files can be retrieved independently. The [API route inventory](10-api-route-inventory.md) is mechanically derived from the stored OpenAPI JSON. The [schema inventory](11-schema-inventory.md) lists every saved proto/XSD. The authored analysis distinguishes protocol requirements, implementation observations, proposed architecture and unresolved behavior.

Repository licenses are archived in [Server LICENSE.md](sources/Server/LICENSE.md), [Server LICENSE.txt](sources/Server/LICENSE.txt), and [ATAK LICENSE.md](sources/atak-civ/LICENSE.md). Public-release notices and individual file notices also matter. Before incorporating or distributing upstream implementation/schema-derived code, choose a licensing approach consistent with the actual files used. This research does not provide a legal conclusion about a future implementation.


<!-- Archived primary sources; exact revisions are recorded in sources/manifest.json. -->
[wire]: sources/atak-civ/takproto/README.txt
[client-proto]: sources/atak-civ/takproto/takmessage.proto
[cot-xsd]: sources/atak-civ/takcot/mitre/CoT%20Base-Event%20Schema%20%20%28PUBLIC%20RELEASE%29.xsd
[openapi]: sources/Server/src/takserver-takcl-core/src/main/openapi/takserver-openapi-spec.json
[guide]: pdfs/TAK_Server_Configuration_Guide.pdf
[cot-guide]: pdfs/CoT_Developers_Guide_2005.pdf
