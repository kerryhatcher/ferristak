# PDF catalog

Downloaded originals retrieved 2026-10-21. These PDFs were checked with pdfinfo and text extraction. Their upstream URLs and SHA-256 values are also recorded in [the source manifest](../sources/manifest.json).

## TAK Server Configuration Guide, version 5.7

- Local PDF: [TAK_Server_Configuration_Guide.pdf](TAK_Server_Configuration_Guide.pdf)
- Original source: [pinned upstream PDF](https://raw.githubusercontent.com/TAK-Product-Center/Server/5187abd46d827d37cfc5708805eced197a837e49/src/docs/TAK_Server_Configuration_Guide.pdf)
- Pages: 67; bytes: 5,516,693.
- Contents: 67-page operational reference covering installation, CoreConfig, TLS/certificates, user authentication, group filtering, federation and group mapping, mission disruption tolerance, retention and administrative tools. PDF metadata reports creation on 2026-03-18. Critical for listener, identity and federation design; not a complete wire/API specification.
- SHA-256: `84a6424b093396f8315c4761999c9bad4019455828e8da6d0fb01b1a114295bd`

## The Developer’s Guide to Cursor on Target, August 2005

- Local PDF: [CoT_Developers_Guide_2005.pdf](CoT_Developers_Guide_2005.pdf)
- Original source: [pinned upstream PDF](https://raw.githubusercontent.com/TAK-Product-Center/atak-civ/9f6893dd657feacc35ec5de03dad721c2e44170e/takcot/mitre/documents/CoT%20Developer%27s%20Guide%20-%202005%200831%20%28PUBLIC%20RELEASE%29.pdf)
- Pages: 19; bytes: 415,943.
- Contents: 19-page MITRE public-release developer introduction to CoT: purpose, event and extension philosophy, identifiers, message routing and original transport assumptions. Includes public release case #06-0249. Historical foundation; its one-event TCP guidance predates TAK persistent streams and binary negotiation.
- SHA-256: `1c70388abef691beb347040fe48e70e85b08fb714c2f2400bf0b83af74eacfe4`

## Located but not downloaded

- **MIL-STD-6090A, Cursor on Target Message Standard**: [official DLA record](https://quicksearch.dla.mil/qsDocDetails.aspx?ident_number=284928). Revision A dated 2025-01-31, 508 pages, Distribution C. Full PDF is controlled; not downloaded or used. Public schema comments are not a replacement for this standard.
- **MITRE Cursor-on-Target Message Router User’s Guide (2010)**: [official landing page](https://www.mitre.org/news-insights/publication/cursor-target-message-router-users-guide-0), [PDF URL](https://www.mitre.org/sites/default/files/pdf/09_4937.pdf). Historical router subscriptions, regular expressions and routing examples. Direct binary retrieval returned HTTP 403, so no local PDF is claimed. Not essential to the TAK-specific streaming design.

The critical modern TAK wire specification is supplied as text rather than PDF: [archived protocol README](../sources/atak-civ/takproto/README.txt). The machine-readable interfaces are preserved as [protobuf and XML schemas](../11-schema-inventory.md) and [OpenAPI JSON](../sources/Server/src/takserver-takcl-core/src/main/openapi/takserver-openapi-spec.json). They were not converted into artificial PDF specifications.
