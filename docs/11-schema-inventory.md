# Schema inventory and differences

The raw schemas are saved unchanged. Use the [manifest](sources/manifest.json) for exact upstream revisions and checksums.

## Important differences

- Client `TakMessage` has control and event fields; server `TakMessage` adds submissionTime (3) and creationTime (4). These additions must not be confused with event timestamps.
- Client `Detail` adds extensionDetails (8), while the archived server detail.proto stops at track (7). A server-generated struct alone therefore cannot represent the full newer client definition.
- Both client and server `CotEvent` have caveat (16) and the release-to field (17), but the server names tag 17 `releaseableTo` while the client uses `releasableTo`. Binary tags match; generated field names and protobuf JSON names differ. Both include updated access comments.
- The public base XSD declares XML attribute `releasableto` (lowercase t), while its own documentation and Common Commo use `releasableTo`. This is a concrete schema/implementation discrepancy. Do not silently normalize case or claim XSD validation guarantees target-client acceptance; preserve input and capture target behavior.
- Mesh and streaming share a payload family but have different outer framing. The separate server federation GeoEvent has its own tags and is not a CotEvent alias.
- Never compile both client and server definitions with the same protobuf package/message names into one undifferentiated code-generation input. Pin a selected compatible client family and generate federation separately.

## Archived protobuf and XSD files

| Repository | Local schema |
|---|---|
| TAK-Product-Center/Server | [CoreConfig.xsd](sources/Server/src/takserver-common/src/main/xsd/CoreConfig.xsd) |
| TAK-Product-Center/Server | [UserAuthenticationFile.xsd](sources/Server/src/takserver-common/src/main/xsd/UserAuthenticationFile.xsd) |
| TAK-Product-Center/Server | [mp.xsd](sources/Server/src/takserver-core/takserver-war/src/main/java/com/bbn/marti/util/missionpackage/mp.xsd) |
| TAK-Product-Center/Server | [binarypayload.proto](sources/Server/src/takserver-protobuf/src/main/proto/binarypayload.proto) |
| TAK-Product-Center/Server | [contact.proto](sources/Server/src/takserver-protobuf/src/main/proto/contact.proto) |
| TAK-Product-Center/Server | [cotevent.proto](sources/Server/src/takserver-protobuf/src/main/proto/cotevent.proto) |
| TAK-Product-Center/Server | [detail.proto](sources/Server/src/takserver-protobuf/src/main/proto/detail.proto) |
| TAK-Product-Center/Server | [fig.proto](sources/Server/src/takserver-protobuf/src/main/proto/fig.proto) |
| TAK-Product-Center/Server | [group.proto](sources/Server/src/takserver-protobuf/src/main/proto/group.proto) |
| TAK-Product-Center/Server | [message.proto](sources/Server/src/takserver-protobuf/src/main/proto/message.proto) |
| TAK-Product-Center/Server | [missionannouncement.proto](sources/Server/src/takserver-protobuf/src/main/proto/missionannouncement.proto) |
| TAK-Product-Center/Server | [precisionlocation.proto](sources/Server/src/takserver-protobuf/src/main/proto/precisionlocation.proto) |
| TAK-Product-Center/Server | [status.proto](sources/Server/src/takserver-protobuf/src/main/proto/status.proto) |
| TAK-Product-Center/Server | [streaminginput.proto](sources/Server/src/takserver-protobuf/src/main/proto/streaminginput.proto) |
| TAK-Product-Center/Server | [takcontrol.proto](sources/Server/src/takserver-protobuf/src/main/proto/takcontrol.proto) |
| TAK-Product-Center/Server | [takmessage.proto](sources/Server/src/takserver-protobuf/src/main/proto/takmessage.proto) |
| TAK-Product-Center/Server | [takv.proto](sources/Server/src/takserver-protobuf/src/main/proto/takv.proto) |
| TAK-Product-Center/Server | [track.proto](sources/Server/src/takserver-protobuf/src/main/proto/track.proto) |
| TAK-Product-Center/atak-civ | [CoT Base-Event Schema  (PUBLIC RELEASE).xsd](sources/atak-civ/takcot/mitre/CoT%20Base-Event%20Schema%20%20%28PUBLIC%20RELEASE%29.xsd) |
| TAK-Product-Center/atak-civ | [CoT Contact Schema (PUBLIC RELEASE).xsd](sources/atak-civ/takcot/mitre/CoT%20Contact%20Schema%20%28PUBLIC%20RELEASE%29.xsd) |
| TAK-Product-Center/atak-civ | [CoT Flow-Tags Schema  (PUBLIC RELEASE).xsd](sources/atak-civ/takcot/mitre/CoT%20Flow-Tags%20Schema%20%20%28PUBLIC%20RELEASE%29.xsd) |
| TAK-Product-Center/atak-civ | [CoT Image Schema  (PUBLIC RELEASE).xsd](sources/atak-civ/takcot/mitre/CoT%20Image%20Schema%20%20%28PUBLIC%20RELEASE%29.xsd) |
| TAK-Product-Center/atak-civ | [CoT Link Schema  (PUBLIC RELEASE).xsd](sources/atak-civ/takcot/mitre/CoT%20Link%20Schema%20%20%28PUBLIC%20RELEASE%29.xsd) |
| TAK-Product-Center/atak-civ | [CoT Remarks Schema  (PUBLIC RELEASE).xsd](sources/atak-civ/takcot/mitre/CoT%20Remarks%20Schema%20%20%28PUBLIC%20RELEASE%29.xsd) |
| TAK-Product-Center/atak-civ | [CoT Request Schema  (PUBLIC RELEASE).xsd](sources/atak-civ/takcot/mitre/CoT%20Request%20Schema%20%20%28PUBLIC%20RELEASE%29.xsd) |
| TAK-Product-Center/atak-civ | [CoT Sensor Schema  (PUBLIC RELEASE).xsd](sources/atak-civ/takcot/mitre/CoT%20Sensor%20Schema%20%20%28PUBLIC%20RELEASE%29.xsd) |
| TAK-Product-Center/atak-civ | [CoT Shape Schema  (PUBLIC RELEASE).xsd](sources/atak-civ/takcot/mitre/CoT%20Shape%20Schema%20%20%28PUBLIC%20RELEASE%29.xsd) |
| TAK-Product-Center/atak-civ | [CoT Spatial Schema  (PUBLIC RELEASE).xsd](sources/atak-civ/takcot/mitre/CoT%20Spatial%20Schema%20%20%28PUBLIC%20RELEASE%29.xsd) |
| TAK-Product-Center/atak-civ | [CoT Track Schema  (PUBLIC RELEASE).xsd](sources/atak-civ/takcot/mitre/CoT%20Track%20Schema%20%20%28PUBLIC%20RELEASE%29.xsd) |
| TAK-Product-Center/atak-civ | [CoT Uid Schema  (PUBLIC RELEASE).xsd](sources/atak-civ/takcot/mitre/CoT%20Uid%20Schema%20%20%28PUBLIC%20RELEASE%29.xsd) |
| TAK-Product-Center/atak-civ | [Chat.xsd](sources/atak-civ/takcot/xsd/Chat.xsd) |
| TAK-Product-Center/atak-civ | [Drawing Shapes - Circle.xsd](sources/atak-civ/takcot/xsd/Drawing%20Shapes%20-%20Circle.xsd) |
| TAK-Product-Center/atak-civ | [Drawing Shapes - Free Form.xsd](sources/atak-civ/takcot/xsd/Drawing%20Shapes%20-%20Free%20Form.xsd) |
| TAK-Product-Center/atak-civ | [Drawing Shapes - Rectangle.xsd](sources/atak-civ/takcot/xsd/Drawing%20Shapes%20-%20Rectangle.xsd) |
| TAK-Product-Center/atak-civ | [Drawing Shapes - Telestration.xsd](sources/atak-civ/takcot/xsd/Drawing%20Shapes%20-%20Telestration.xsd) |
| TAK-Product-Center/atak-civ | [Geo Fence.xsd](sources/atak-civ/takcot/xsd/Geo%20Fence.xsd) |
| TAK-Product-Center/atak-civ | [Marker - 2525.xsd](sources/atak-civ/takcot/xsd/Marker%20-%202525.xsd) |
| TAK-Product-Center/atak-civ | [Marker - Icon Set.xsd](sources/atak-civ/takcot/xsd/Marker%20-%20Icon%20Set.xsd) |
| TAK-Product-Center/atak-civ | [Marker - Spot.xsd](sources/atak-civ/takcot/xsd/Marker%20-%20Spot.xsd) |
| TAK-Product-Center/atak-civ | [Range & Bearing - Bullseye.xsd](sources/atak-civ/takcot/xsd/Range%20%26%20Bearing%20-%20Bullseye.xsd) |
| TAK-Product-Center/atak-civ | [Range & Bearing - Circle.xsd](sources/atak-civ/takcot/xsd/Range%20%26%20Bearing%20-%20Circle.xsd) |
| TAK-Product-Center/atak-civ | [Range & Bearing - Line.xsd](sources/atak-civ/takcot/xsd/Range%20%26%20Bearing%20-%20Line.xsd) |
| TAK-Product-Center/atak-civ | [Route.xsd](sources/atak-civ/takcot/xsd/Route.xsd) |
| TAK-Product-Center/atak-civ | [__cep.xsd](sources/atak-civ/takcot/xsd/details/__cep.xsd) |
| TAK-Product-Center/atak-civ | [__chat.xsd](sources/atak-civ/takcot/xsd/details/__chat.xsd) |
| TAK-Product-Center/atak-civ | [__chatreceipt.xsd](sources/atak-civ/takcot/xsd/details/__chatreceipt.xsd) |
| TAK-Product-Center/atak-civ | [__geofence.xsd](sources/atak-civ/takcot/xsd/details/__geofence.xsd) |
| TAK-Product-Center/atak-civ | [__group.xsd](sources/atak-civ/takcot/xsd/details/__group.xsd) |
| TAK-Product-Center/atak-civ | [__lob.xsd](sources/atak-civ/takcot/xsd/details/__lob.xsd) |
| TAK-Product-Center/atak-civ | [__milsym.xsd](sources/atak-civ/takcot/xsd/details/__milsym.xsd) |
| TAK-Product-Center/atak-civ | [__remotecontrol.xsd](sources/atak-civ/takcot/xsd/details/__remotecontrol.xsd) |
| TAK-Product-Center/atak-civ | [__serverdestination.xsd](sources/atak-civ/takcot/xsd/details/__serverdestination.xsd) |
| TAK-Product-Center/atak-civ | [__signalInfo.xsd](sources/atak-civ/takcot/xsd/details/__signalInfo.xsd) |
| TAK-Product-Center/atak-civ | [__video.xsd](sources/atak-civ/takcot/xsd/details/__video.xsd) |
| TAK-Product-Center/atak-civ | [archive.xsd](sources/atak-civ/takcot/xsd/details/archive.xsd) |
| TAK-Product-Center/atak-civ | [attachment_list.xsd](sources/atak-civ/takcot/xsd/details/attachment_list.xsd) |
| TAK-Product-Center/atak-civ | [chatgrp.xsd](sources/atak-civ/takcot/xsd/details/chatgrp.xsd) |
| TAK-Product-Center/atak-civ | [color.xsd](sources/atak-civ/takcot/xsd/details/color.xsd) |
| TAK-Product-Center/atak-civ | [contact.xsd](sources/atak-civ/takcot/xsd/details/contact.xsd) |
| TAK-Product-Center/atak-civ | [creator.xsd](sources/atak-civ/takcot/xsd/details/creator.xsd) |
| TAK-Product-Center/atak-civ | [emergency.xsd](sources/atak-civ/takcot/xsd/details/emergency.xsd) |
| TAK-Product-Center/atak-civ | [environment.xsd](sources/atak-civ/takcot/xsd/details/environment.xsd) |
| TAK-Product-Center/atak-civ | [extrudeMode.xsd](sources/atak-civ/takcot/xsd/details/extrudeMode.xsd) |
| TAK-Product-Center/atak-civ | [fileshare.xsd](sources/atak-civ/takcot/xsd/details/fileshare.xsd) |
| TAK-Product-Center/atak-civ | [fillColor.xsd](sources/atak-civ/takcot/xsd/details/fillColor.xsd) |
| TAK-Product-Center/atak-civ | [height.xsd](sources/atak-civ/takcot/xsd/details/height.xsd) |
| TAK-Product-Center/atak-civ | [height_unit.xsd](sources/atak-civ/takcot/xsd/details/height_unit.xsd) |
| TAK-Product-Center/atak-civ | [hierarchy.xsd](sources/atak-civ/takcot/xsd/details/hierarchy.xsd) |
| TAK-Product-Center/atak-civ | [labels_on.xsd](sources/atak-civ/takcot/xsd/details/labels_on.xsd) |
| TAK-Product-Center/atak-civ | [link.xsd](sources/atak-civ/takcot/xsd/details/link.xsd) |
| TAK-Product-Center/atak-civ | [mission.xsd](sources/atak-civ/takcot/xsd/details/mission.xsd) |
| TAK-Product-Center/atak-civ | [precisionlocation.xsd](sources/atak-civ/takcot/xsd/details/precisionlocation.xsd) |
| TAK-Product-Center/atak-civ | [remarks.xsd](sources/atak-civ/takcot/xsd/details/remarks.xsd) |
| TAK-Product-Center/atak-civ | [shape.xsd](sources/atak-civ/takcot/xsd/details/shape.xsd) |
| TAK-Product-Center/atak-civ | [status.xsd](sources/atak-civ/takcot/xsd/details/status.xsd) |
| TAK-Product-Center/atak-civ | [strokeColor.xsd](sources/atak-civ/takcot/xsd/details/strokeColor.xsd) |
| TAK-Product-Center/atak-civ | [strokeWeight.xsd](sources/atak-civ/takcot/xsd/details/strokeWeight.xsd) |
| TAK-Product-Center/atak-civ | [takv.xsd](sources/atak-civ/takcot/xsd/details/takv.xsd) |
| TAK-Product-Center/atak-civ | [track.xsd](sources/atak-civ/takcot/xsd/details/track.xsd) |
| TAK-Product-Center/atak-civ | [uid.xsd](sources/atak-civ/takcot/xsd/details/uid.xsd) |
| TAK-Product-Center/atak-civ | [usericon.xsd](sources/atak-civ/takcot/xsd/details/usericon.xsd) |
| TAK-Product-Center/atak-civ | [point.xsd](sources/atak-civ/takcot/xsd/event/point.xsd) |
| TAK-Product-Center/atak-civ | [contact.proto](sources/atak-civ/takproto/contact.proto) |
| TAK-Product-Center/atak-civ | [cotevent.proto](sources/atak-civ/takproto/cotevent.proto) |
| TAK-Product-Center/atak-civ | [detail.proto](sources/atak-civ/takproto/detail.proto) |
| TAK-Product-Center/atak-civ | [group.proto](sources/atak-civ/takproto/group.proto) |
| TAK-Product-Center/atak-civ | [precisionlocation.proto](sources/atak-civ/takproto/precisionlocation.proto) |
| TAK-Product-Center/atak-civ | [status.proto](sources/atak-civ/takproto/status.proto) |
| TAK-Product-Center/atak-civ | [takcontrol.proto](sources/atak-civ/takproto/takcontrol.proto) |
| TAK-Product-Center/atak-civ | [takmessage.proto](sources/atak-civ/takproto/takmessage.proto) |
| TAK-Product-Center/atak-civ | [takv.proto](sources/atak-civ/takproto/takv.proto) |
| TAK-Product-Center/atak-civ | [track.proto](sources/atak-civ/takproto/track.proto) |
