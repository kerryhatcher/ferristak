# CoT data model and message semantics

## Core event

**Documented:** the publicly released base schema defines an event containing a point and optional detail. The schema includes historical clarifications and a 2023 update referencing MIL-STD-6090. It is more useful than a simplistic example copied from a blog. Sources: [base XSD][cot-xsd], [protobuf mapping][event-proto].

| Field | Meaning / handling |
|---|---|
| version | XML schema version; commonly 2.0; not wire protocol version |
| uid | Entity/event identifier; keep as an opaque string |
| type | Hierarchical, case-sensitive semantic type; not always a MIL-STD-2525 symbol |
| time | Event transmission/report time |
| start | Start of validity |
| stale | End of validity for this report |
| how | Method/source of production; preserve opaque values |
| access, caveat, releasableTo | Access/dissemination markings; preserve exactly and define policy |
| qos, opex | Optional quality and operational/exercise metadata |
| point.lat / lon | WGS-84 geographic coordinates, degrees |
| point.hae | Height above ellipsoid, meters; not automatically mean sea level |
| point.ce / le | Circular/linear error, meters; unknown sentinel handling matters |
| detail | Extensible application metadata, often decisive for application behavior |

The client protobuf uses 999999 for unknown hae/ce/le. The updated client comments say access is now required by the military standard; legacy absence/empty should be interpreted as Undefined, and Undefined is omitted in protobuf for compactness. This is a statement in the public schema comments, **not** an independent review of the controlled standard. Source: [CotEvent fields and comments][event-proto].

Illustrative legacy-compatible message, not a captured fixture:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<event version="2.0" uid="demo-device" type="a-f-G-U-C" time="2026-10-21T15:00:00Z" start="2026-10-21T15:00:00Z" stale="2026-10-21T15:02:00Z" how="m-g" access="Undefined">
  <point lat="32.8407" lon="-83.6324" hae="999999" ce="999999" le="999999"/>
  <detail><contact callsign="DEMO" endpoint="*:-1:stcp"/><__group name="Cyan" role="Team Member"/></detail>
</event>
```

Do not copy the display team in `__group` into authorization memberships. The display data is supplied by the client; server channels/groups come from authenticated policy.

## Protobuf field contract

The version 1 client package is `atakmap.commoncommo.protobuf.v1`. `TakMessage.takControl` has tag 1 and `.cotEvent` tag 2; both may be absent. Control-only messages are valid. `CotEvent` mapping:

| Tags | Fields |
|---|---|
| 1–5 | type, access, qos, opex, uid |
| 6–8 | sendTime, startTime, staleTime: uint64 milliseconds since Unix epoch |
| 9 | how |
| 10–14 | lat, lon, hae, ce, le: double |
| 15 | detail |
| 16–17 | caveat, releasableTo in the archived client schema |

Sources: [TakMessage][client-proto], [CotEvent][event-proto]. Proto3 scalar defaults are not evidence that a field was semantically supplied. Validate conversions and time range explicitly; epoch conversion can overflow narrower Rust timestamp representations. An absent coordinate cannot safely be treated as proof of a real zero coordinate.

The archived base XSD declares `releasableto` while Common Commo reads/writes `releasableTo`; the server protobuf spells tag 17 `releaseableTo` and the client spells it `releasableTo`. Binary tags match, but XML and generated JSON names require deliberate handling. See [schema discrepancies](11-schema-inventory.md).

## Lossless detail conversion

Typed detail fields are contact (2), group (3), precisionLocation (4), status (5), takv (6), and track (7); xmlDetail is field 1. The newer client adds repeated extensionDetails (8). xmlDetail contains UTF-8 children of detail, **without** a detail wrapper or XML declaration. Source: [Detail definition and conversion rules][detail-proto].

Convert an entire element to a typed message only when every attribute/child and occurrence can be represented. If mapping fails, an unknown attribute exists, or duplicate occurrences exceed typed capacity, keep the entire element in xmlDetail and omit the typed counterpart. On decoding, restore the detail wrapper and merge typed equivalents; if XML already contains that element, leave the XML and ignore the typed duplicate. This prevents subtle plugin data loss. The C++ implementation checks for unmapped attributes and children before converting. Sources: [mapping rules][detail-proto], [mapping implementation][cot-impl].

**Proposed:** retain raw wire payload bytes alongside an indexed representation. Ordinary generated protobuf structs should not be assumed to preserve all unknown fields through decode/re-encode. Same-format forwarding can reuse original bytes when no mutation is needed. A routing or sanitization mutation requires a deliberate preservation strategy. Do not treat opaque XML as trusted policy.

## Application messages to support

| Family | Evidence / fields | Server responsibilities |
|---|---|---|
| Presence and position | contact, track, precisionlocation, takv, status, __group | Contact/session association, permitted latest-state replay, expiry |
| GeoChat | __chat, chatgrp, remarks, __chatreceipt schemas | Directed delivery, room semantics and receipts preserved |
| Drawings and routes | shape, link, style fields; drawing and Route XSDs | Preserve full detail, avoid point-only reconstruction |
| Emergency | emergency type and cancellation | Explicit repeat/cancel policy; never coalesce indiscriminately |
| Files | fileshare, ackrequest, acknowledgment details | Deliver transfer announcements and receipts plus HTTPS content |
| Missions | mission detail and marti destination | Membership-authorized delivery plus HTTP persisted state |
| Connection control | protocol offer/request/response, ping/pong | Handle within connection state machine |

Primary schemas are archived under [TAK XSDs](sources/atak-civ/takcot/xsd/) and [MITRE XSDs](sources/atak-civ/takcot/mitre/). File construction and destination code: [cotmessage.cpp][cot-impl]. Emergency repeater examples: [CoreConfig][config]. The existence of an XSD does not prove every app release emits that exact layout.

## Directed routing and persistence

Client `setTAKServerRecipients` emits `detail/marti/dest callsign="..."`; `setTAKServerMissionRecipient` emits `dest mission="..."`. A missing/empty recipient list removes marti addressing and becomes broadcast in that client code. The code also supports peer `__dest uid`. Source: [destination constructors][cot-impl].

**Proposed:** resolve destinations using session/contact indices within the sender's authorization scope. Callsigns are mutable and can collide. A destination hint never grants access. Define and document policy for collisions, unknown recipients, multiple sessions, disconnects, echoes, and malformed destinations before implementation.

Keep three distinct stores: transient connection presence, latest entity state by scoped UID, and append-only event history. Repeated UID reports update latest state; they are not automatically duplicates. Retain stale reports in history only under an explicit retention policy and do not replay them as live presence. Ordering across multiple publishers with the same UID and skewed clocks remains a product decision requiring fixtures.


<!-- Archived primary sources; exact revisions are recorded in sources/manifest.json. -->
[client-proto]: sources/atak-civ/takproto/takmessage.proto
[event-proto]: sources/atak-civ/takproto/cotevent.proto
[detail-proto]: sources/atak-civ/takproto/detail.proto
[cot-xsd]: sources/atak-civ/takcot/mitre/CoT%20Base-Event%20Schema%20%20%28PUBLIC%20RELEASE%29.xsd
[cot-impl]: sources/atak-civ/commoncommo/core/impl/cotmessage.cpp
[config]: sources/Server/src/takserver-core/example/CoreConfig.example.xml
