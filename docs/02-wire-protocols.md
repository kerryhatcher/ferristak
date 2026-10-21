# Wire protocols and stream state

## Transport and framing contracts

**Documented:** TAK protocol version 0 means legacy CoT XML. Version 1 means the protobuf payload family. These version numbers are independent of CoT XML's event version 2.0, ATAK application releases, and federation versions. The protocol specification remains authoritative even when a library calls all traffic simply “CoT.” Source: [official wire specification][wire].

| Mode | Message framing | Boundary |
|---|---|---|
| Legacy UDP | One CoT XML document | Datagram boundary |
| Legacy directed TCP | Connect, send one event, close | Connection/event end |
| Legacy server streaming | XML declaration, newline, event; repeat | End of event element |
| Binary mesh UDP / directed TCP | 0xBF + unsigned varint(version) + 0xBF + serialized TakMessage | Datagram / single message |
| Negotiated binary server stream | 0xBF + unsigned varint(payload byte length) + serialized TakMessage | Explicit length |

Version 1 mesh begins `BF 01 BF`; version 1 streaming does **not** use that header. For a hypothetical 150-byte streaming payload, its prefix is `BF 96 01`, followed by exactly 150 payload bytes. Payload length excludes the header. The payload is a serialized `TakMessage`, not a bare `CotEvent` and not a gRPC envelope. Source: [framing and varint sections][wire].

Unsigned varints use seven data bits per byte, low groups first, with the high bit indicating continuation. Bound both encoded width and decoded value before allocation. Handle incomplete headers, incomplete bodies, multiple frames in a read, zero/oversized lengths, and malformed encodings. Limits and disconnect policy are **proposed** implementation decisions; the README is not a complete resource-limit specification.

## Legacy XML serialization

The wire README describes messages with an XML declaration, newline, then complete event. Do not append arbitrary newline separators after `</event>`; the next document begins immediately. A TCP read, TLS record, or write call is never a message boundary. Consume multiple XML documents incrementally.

**Proposed parser behavior:** identify the root XML document with bounded tokenization and preserve unread bytes. A bare substring search for `</event>` is unsafe around CDATA/comments and unsuitable as the only malformed-input defense. Support the separate authentication root as well as events. Keep an incomplete document across reads. Enforce depth, bytes, text and attribute limits, reject DTD/entity declarations, and reject unsupported encodings. Sources: [legacy protocol][wire], [client stream decoder][stream-impl].

## Authentication precedes application traffic

If credentials are configured, the client sends an authentication XML document first:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<auth><cot username="example" password="REDACTED" uid="device-uid"/></auth>
```

This document is not a CoT event. The client generates it in `generateAuthDoc`; mTLS authentication and this username/password message are separate mechanisms. Do not persist or log this document as an event. Failed required authentication closes the connection. Sources: [client implementation][stream-impl], [negotiation specification][wire].

## Protocol negotiation state machine

1. Establish TCP/TLS and required authentication. Both directions initially use XML.
2. The server may send one offer event of type `t-x-takp-v`. It contains `detail/TakControl/TakProtocolSupport` with one or more supported `version` values.
3. An interested client sends `t-x-takp-q`, containing exactly one `TakRequest version="1"`. It must have observed the server offer first. The transaction event UID is reused from the offer.
4. After sending the request, the client stops sending ordinary XML and continues receiving XML while awaiting the response.
5. The server sends `t-x-takp-r` containing exactly one `TakResponse status="true"` or `status="false"`.
6. On true, that response is the last XML message from the server; all subsequent data in **both directions** uses the negotiated binary stream framing. On false, XML exchange resumes and negotiation may be retried.
7. A client must wait at least one minute for a response. A timeout leaves the state indeterminate: disconnect and reconnect rather than guessing a codec.

The server must inspect for requests from the offer until at least a minute after it, and give retry time following a denial. It must not repeat the offer per connection. Source: [Streaming Connection Protocol Negotiation][wire].

**Proposed writer invariant:** one task owns each connection's ordered output. Serialize the accepted XML response through that writer, switch its codec at that boundary, and preserve already-read binary bytes in the reader buffer. An XML reply and the first binary frame can be coalesced into one socket read. A separate broadcast writer must never race this transition.

## Detail extension negotiation

A server offer can advertise `DetailExt id="N"`, or a single `DetailExt supportsAll="true"`. Each element uses one of those attributes. A client request can advertise explicit supported IDs; it cannot advertise the wildcard form. Extensions replace entire detail element trees and IDs must be registered with TPC. A sender must not use an extension the recipient has not advertised. The protocol warns that forwarding servers can nevertheless expose clients to unfamiliar extensions; incomplete decoding must be visible. Source: [extension rules][wire], [Detail field 8][detail-proto].

**Proposed:** initially advertise no registered extensions. Never advertise supportsAll unless opaque preservation is complete through persistence, routing, and reserialization. Do not transcode an unknown extension to XML while implying nothing was lost. See [schema differences](11-schema-inventory.md).

## Mesh version selection

Mesh participants advertise TakControl at least every 60 seconds. Peers track supported version ranges and detail extension sets. After two minutes without a control announcement, support reverts to the version of the latest presence-maintaining message. Broadcasts use the highest version common to all known participants; if none, XML version 0 is the fallback. Version changes trigger an immediate control announcement. Even during XML broadcasts, control advertisements use the sender's lowest supported version above zero. Source: [Mesh Network Protocol Negotiation][wire].

This logic belongs in a LAN gateway if that is included in scope. A WAN streaming server does not need to pretend every connected client is one multicast mesh.

## Keepalive and disconnect

Client source identifies ping `t-x-c-t` and pong `t-x-c-t-r`, and emits pings after receive inactivity. Treat these as connection controls, not chat or shared map objects. The client's timeout constants are implementation behavior, not protocol-wide fixed requirements. Define idle, handshake, negotiation, write and shutdown timeouts separately. Sources: [CoT control types][cot-impl], [stream manager][stream-impl].


<!-- Archived primary sources; exact revisions are recorded in sources/manifest.json. -->
[wire]: sources/atak-civ/takproto/README.txt
[detail-proto]: sources/atak-civ/takproto/detail.proto
[cot-impl]: sources/atak-civ/commoncommo/core/impl/cotmessage.cpp
[stream-impl]: sources/atak-civ/commoncommo/core/impl/streamingsocketmanagement.cpp
