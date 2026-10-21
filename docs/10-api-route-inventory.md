# Archived API route inventory

Derived from the [pinned OpenAPI JSON](sources/Server/src/takserver-takcl-core/src/main/openapi/takserver-openapi-spec.json). This is a reference inventory, not an implementation status report. Legacy servlet routes under `/Marti/sync/*` are additional; see [HTTP research](04-http-interfaces.md). Generated media types and success-only responses can be incomplete.

OpenAPI version: `3.0.1`; route paths: 290.

Operations: 380. The JSON also includes 243 component schemas. Follow each named schema in the saved JSON for exact fields and enums.

## /Marti/api

### GET — getAllSearches

Tags: cot-query-api.

Responses: `200` */* → ApiResponseListCotSearch.

## /Marti/api/activeconnections

### GET — getActiveConnections

Tags: federation-api.

Responses: `200` */* → ApiResponseListConnectionInfoSummary.

## /Marti/api/authentication/config

### GET — getAuthConfig

Tags: security-authentication-api.

Responses: `200` */* → ApiResponseAuthenticationConfigInfo.

### PUT — modifyAuthConfig

Tags: security-authentication-api.

Request body (required): `application/json` → AuthenticationConfigInfo.

Responses: `200` */* → ApiResponseString.

### POST — testAuthConfig

Tags: security-authentication-api.

Responses: `200` */* → ApiResponseString.

## /Marti/api/cachedConfig

### GET — getCachedCoreConfig

Tags: config-api.

Responses: `200` */* → Configuration.

## /Marti/api/cachedInputConfig

### GET — getCachedInputConfig

Tags: config-api.

Responses: `200` */* → array[Input].

## /Marti/api/certadmin/cert

### GET — getAll_1

Tags: cert-manager-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| username | query | False | string |  |

Responses: `200` */* → ApiResponseListTakCert.

## /Marti/api/certadmin/cert/active

### GET — getActive

Tags: cert-manager-admin-api.

Responses: `200` */* → ApiResponseListTakCert.

## /Marti/api/certadmin/cert/delete/{ids}

### DELETE — deleteCertificates

Tags: cert-manager-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| ids | path | True | string |  |

Responses: `200` */* → string.

## /Marti/api/certadmin/cert/download/{ids}

### GET — downloadCertificates

Tags: cert-manager-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| ids | path | True | string |  |

Responses: `200` */* → string (byte).

## /Marti/api/certadmin/cert/expired

### GET — getExpired

Tags: cert-manager-admin-api.

Responses: `200` */* → ApiResponseListTakCert.

## /Marti/api/certadmin/cert/replaced

### GET — getReplaced

Tags: cert-manager-admin-api.

Responses: `200` */* → ApiResponseListTakCert.

## /Marti/api/certadmin/cert/revoke/{ids}

### DELETE — revokeCertificates

Tags: cert-manager-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| ids | path | True | string |  |

Responses: `200` */* → string.

## /Marti/api/certadmin/cert/revoked

### GET — getRevoked

Tags: cert-manager-admin-api.

Responses: `200` */* → ApiResponseListTakCert.

## /Marti/api/certadmin/cert/{hash}

### GET — getCertificate

Tags: cert-manager-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| hash | path | True | string |  |

Responses: `200` */* → ApiResponseTakCert.

### DELETE — revokeCertificate

Tags: cert-manager-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| hash | path | True | string |  |

Responses: `200` */* → ApiResponseTakCert.

## /Marti/api/certadmin/cert/{hash}/download

### GET — downloadCertificate

Tags: cert-manager-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| hash | path | True | string |  |

Responses: `200` */* → string.

## /Marti/api/citrap

### GET — searchReports

Tags: ci-trap-report-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| keywords | query | False | string |  |
| bbox | query | False | string |  |
| startTime | query | False | string |  |
| endTime | query | False | string |  |
| maxReportCount | query | False | string |  |
| type | query | False | string |  |
| callsign | query | False | string |  |
| subscribe | query | False | string |  |
| clientUid | query | False | string |  |

Responses: `200` */* → string.

### POST — postReport

Tags: ci-trap-report-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| clientUid | query | True | string |  |

Request body (required): `application/json` → string (byte).

Responses: `200` */* → string.

## /Marti/api/citrap/{id}

### GET — getReport

Tags: ci-trap-report-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| id | path | True | string |  |
| clientUid | query | True | string |  |

Responses: `200` */* → string.

### PUT — putReport

Tags: ci-trap-report-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| id | path | True | string |  |
| clientUid | query | True | string |  |

Request body (required): `application/json` → string (byte).

Responses: `200` */* → string.

### DELETE — deleteReport

Tags: ci-trap-report-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| id | path | True | string |  |
| clientUid | query | True | string |  |

Responses: `200` */* → string.

## /Marti/api/citrap/{id}/attachment

### POST — addAttachment

Tags: ci-trap-report-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| id | path | True | string |  |
| clientUid | query | True | string |  |

Request body (required): `application/json` → string (byte).

Responses: `200` */* → string.

## /Marti/api/clearFederationEvents

### GET — clearDisruptionData

Tags: federation-api.

Responses: `200` OK.

## /Marti/api/clientEndPoints

### GET — getClientEndpoints

Tags: contact-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| secAgo | query | False | integer (int64) | 0 |
| showCurrentlyConnectedClients | query | False | string | "false" |
| showMostRecentOnly | query | False | string | "false" |
| group | query | False | array[string] |  |

Responses: `200` */* → ApiResponseListClientEndpoint.

## /Marti/api/config

### GET — getCoreConfig

Tags: config-api.

Responses: `200` */* → Configuration.

## /Marti/api/contacts/all

### GET — getAllContacts

Tags: contacts-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| sortBy | query | False | string | "CALLSIGN" |
| direction | query | False | string | "ASCENDING" |
| noFederates | query | False | boolean | false |

Responses: `200` */* → array[RemoteSubscription].

## /Marti/api/cops

### GET — getAllCopMissions

Tags: cop-view-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| path | query | False | string |  |
| offset | query | False | integer (int32) |  |
| size | query | False | integer (int32) |  |

Responses: `200` */* → ApiResponseListMission.

## /Marti/api/cops/hierarchy

### GET — getHierarchy

Tags: cop-view-api.

Responses: `200` */* → ApiResponseSetCopHierarchyNode.

## /Marti/api/cot

### GET — getCotEvents

Tags: cot-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uids | query | True | array[string] |  |

Responses: `200` */* → string.

## /Marti/api/cot/matchUid

### GET — getCotEventsByTimeAndBbox_1

Tags: cot-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| search | query | False | string | " " |

Responses: `200` */* → array[string].

## /Marti/api/cot/sa

### GET — getCotEventsByTimeAndBbox

Tags: cot-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| start | query | True | string (date-time) |  |
| end | query | True | string (date-time) |  |
| left | query | False | number (double) |  |
| bottom | query | False | number (double) |  |
| right | query | False | number (double) |  |
| top | query | False | number (double) |  |

Responses: `200` */* → string.

## /Marti/api/cot/search/date

### GET — getDate

Tags: cot-query-api.

Responses: `200` */* → string (date-time).

## /Marti/api/cot/search/{id}

### GET — getSearch

Tags: cot-query-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| id | path | True | string |  |

Responses: `200` */* → ApiResponseListCotSearch.

## /Marti/api/cot/xml/{uid}

### GET — getCotEvent

Tags: cot-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Responses: `200` */* → string.

## /Marti/api/cot/xml/{uid}/all

### GET — getAllCotEvents

Tags: cot-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |
| secago | query | False | integer (int64) |  |
| start | query | False | string (date-time) |  |
| end | query | False | string (date-time) |  |

Responses: `200` */* → string.

## /Marti/api/database/cotCount

### GET — getDatabaseCotCounts

Tags: submission-api.

Responses: `200` */* → ApiResponseMapStringInteger.

## /Marti/api/datafeeds

### GET — getDataFeeds

Tags: data-feed-api.

Responses: `200` */* → ApiResponseListDataFeed.

### POST — createDataFeed

Tags: submission-api.

Request body (required): `application/json` → DataFeed.

Responses: `200` */* → ApiResponseDataFeed.

## /Marti/api/datafeeds/bounds/polygon

### GET — getDataFeedsInPolygon

Tags: data-feed-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| points | query | True | array[string] |  |

Responses: `200` */* → ApiResponseListString.

## /Marti/api/datafeeds/bounds/{bbox}

### GET — getDataFeedsInBbox

Tags: data-feed-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| bbox | path | True | string |  |

Responses: `200` */* → ApiResponseListString.

## /Marti/api/datafeeds/predicate

### PUT — updatePredicateDataFeed

Tags: data-feed-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| updateGroups | query | False | boolean | false |

Request body (required): `application/json` → PredicateDataFeed.

Responses: `200` */* → ApiResponsePredicateDataFeed.

### POST — createPredicateDataFeed

Tags: data-feed-api.

Request body (required): `application/json` → PredicateDataFeed.

Responses: `200` */* → ApiResponsePredicateDataFeed.

## /Marti/api/datafeeds/predicate/{feedGuid}

### DELETE — deletePredicateDataFeed

Tags: data-feed-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| feedGuid | path | True | string |  |

Responses: `200` */* → ApiResponsePredicateDataFeed.

## /Marti/api/datafeeds/predicate/{feedUuid}

### GET — getPredicateDataFeedByGuid

Tags: data-feed-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| feedUuid | path | True | string |  |

Responses: `200` */* → ApiResponsePredicateDataFeed.

## /Marti/api/datafeeds/stats

### GET — getStatsForDataFeed

Tags: data-feed-api.

Responses: `200` */* → ApiResponseListDataFeedStats.

## /Marti/api/datafeeds/stats/{uuid}

### GET — getStatsForDataFeed_1

Tags: data-feed-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uuid | path | True | string |  |

Responses: `200` */* → ApiResponseDataFeedStats.

## /Marti/api/datafeeds/{name}

### GET — getDataFeed

Tags: submission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → ApiResponseDataFeed.

### PUT — modifyDataFeed

Tags: submission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Request body (required): `application/json` → DataFeed.

Responses: `200` */* → ApiResponseDataFeed.

### DELETE — deleteDataFeed

Tags: submission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → ApiResponseDataFeed.

## /Marti/api/datafeeds/{uuid}/cots/{cot_type}

### GET — getCotsForDataFeedByCotType

Tags: data-feed-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uuid | path | True | string |  |
| cot_type | path | True | string |  |

Responses: `200` */* → ApiResponseListString.

## /Marti/api/datafeeds/{uuid}/cots_types

### GET — getExistingCotTypesForDataFeed

Tags: data-feed-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uuid | path | True | string |  |

Responses: `200` */* → ApiResponseListString.

## /Marti/api/device/profile

### GET — getAllProfile

Tags: profile-admin-api.

Responses: `200` */* → ApiResponseListProfile.

## /Marti/api/device/profile/connection

### GET — getConnectionTimeProfiles

Tags: profile-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| syncSecago | query | True | integer (int64) |  |
| clientUid | query | True | string |  |

Responses: `200` */* → string (byte).

## /Marti/api/device/profile/directories

### GET — getValidDirectories

Tags: profile-admin-api.

Responses: `200` */* → ApiResponseListString.

## /Marti/api/device/profile/tool/{toolName}

### GET — getToolProfiles

Tags: profile-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| toolName | path | True | string |  |
| syncSecago | query | False | integer (int64) | -1 |
| clientUid | query | True | string |  |

Responses: `200` */* → string (byte).

## /Marti/api/device/profile/tool/{toolName}/file

### GET — deviceGetProfileDirectoryContent

Tags: profile-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| toolName | path | True | string |  |
| relativePath | query | True | array[string] |  |
| syncSecago | query | False | integer (int64) | -1 |
| clientUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/device/profile/{id}

### DELETE — deleteProfile

Tags: profile-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| id | path | True | integer (int64) |  |

Responses: `200` */* → string.

## /Marti/api/device/profile/{name}

### GET — getProfile

Tags: profile-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → ApiResponseProfile.

### PUT — updateProfile

Tags: profile-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Request body (required): `application/json` → Profile.

Responses: `200` */* → string.

### POST — createProfile

Tags: profile-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| group | query | False | array[string] | [] |

Responses: `200` */* → string.

## /Marti/api/device/profile/{name}/directories

### GET — getDirectories

Tags: profile-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → ApiResponseListProfileDirectory.

### DELETE — deleteDirectories

Tags: profile-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → string.

## /Marti/api/device/profile/{name}/directories/{directories}

### PUT — updateDirectories

Tags: profile-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| directories | path | True | array[string] |  |

Responses: `200` */* → string.

## /Marti/api/device/profile/{name}/file

### PUT — addFile

Tags: profile-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| filename | query | True | string |  |

Request body (required): `application/json` → string (byte).

Responses: `200` */* → ApiResponseProfileFile.

## /Marti/api/device/profile/{name}/file/{id}

### GET — getFile_2

Tags: profile-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| id | path | True | integer (int64) |  |

Responses: `200` */* → string (byte).

### DELETE — deleteFile_1

Tags: profile-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| id | path | True | integer (int64) |  |

Responses: `200` */* → string.

## /Marti/api/device/profile/{name}/files

### GET — getFiles

Tags: profile-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → ApiResponseListProfileFile.

## /Marti/api/device/profile/{name}/missionpackage

### GET — getProfileMp

Tags: profile-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → string (byte).

### HEAD — headProfileMp

Tags: profile-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` OK.

## /Marti/api/device/profile/{name}/send

### POST — sendProfile

Tags: profile-admin-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Request body (required): `application/json` → array[string].

Responses: `200` */* → string.

## /Marti/api/excheck/checklist

### POST — createChecklist

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| clientUid | query | True | string |  |
| defaultRole | query | False | string |  |

Request body (required): `application/json` → string.

Responses: `200` */* → string.

## /Marti/api/excheck/checklist/active

### GET — getChecklist_1

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| clientUid | query | True | string |  |

Responses: `200` */* → string.

## /Marti/api/excheck/checklist/{checklistUid}

### GET — getChecklist

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| checklistUid | path | True | string |  |
| clientUid | query | False | string |  |
| secago | query | False | integer (int64) |  |
| token | query | False | string |  |

Responses: `200` */* → string.

### DELETE — deleteChecklist

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| checklistUid | path | True | string |  |
| clientUid | query | True | string |  |

Responses: `200` */* → string.

## /Marti/api/excheck/checklist/{checklistUid}/mission/{missionName}

### PUT — addMissionReferenceToChecklist

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| checklistUid | path | True | string |  |
| missionName | path | True | string |  |
| clientUid | query | True | string |  |
| password | query | False | string |  |

Responses: `200` */* → string.

### DELETE — removeMissionReferenceFromChecklist

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| checklistUid | path | True | string |  |
| missionName | path | True | string |  |
| clientUid | query | True | string |  |

Responses: `200` */* → string.

## /Marti/api/excheck/checklist/{checklistUid}/status

### GET — getChecklistStatus

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| checklistUid | path | True | string |  |
| token | query | False | string |  |
| clientUid | query | False | string |  |

Responses: `200` */* → string.

## /Marti/api/excheck/checklist/{checklistUid}/task/{taskUid}

### GET — getChecklistTask

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| checklistUid | path | True | string |  |
| taskUid | path | True | string |  |
| clientUid | query | True | string |  |

Responses: `200` */* → string.

### PUT — addEditChecklistTask

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| checklistUid | path | True | string |  |
| taskUid | path | True | string |  |
| clientUid | query | True | string |  |

Request body (required): `application/json` → string.

Responses: `200` */* → string.

### DELETE — deleteChecklistTask

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| checklistUid | path | True | string |  |
| taskUid | path | True | string |  |
| clientUid | query | True | string |  |

Responses: `200` */* → string.

## /Marti/api/excheck/template

### POST — postTemplate

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| clientUid | query | True | string |  |
| callsign | query | False | string |  |
| name | query | False | string |  |
| description | query | False | string |  |

Responses: `200` */* → string.

## /Marti/api/excheck/template/{templateUid}

### GET — getTemplate

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| templateUid | path | True | string |  |
| clientUid | query | True | string |  |

Responses: `200` */* → string.

### DELETE — deleteTemplate

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| templateUid | path | True | string |  |
| clientUid | query | True | string |  |

Responses: `200` */* → string.

## /Marti/api/excheck/template/{templateUid}/task/{taskUid}

### GET — getTemplateTask

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| templateUid | path | True | string |  |
| taskUid | path | True | string |  |
| clientUid | query | True | string |  |

Responses: `200` */* → string.

### PUT — addEditTemplateTask

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| templateUid | path | True | string |  |
| taskUid | path | True | string |  |
| clientUid | query | True | string |  |

Request body (required): `application/json` → string.

Responses: `200` */* → string.

### DELETE — deleteTemplateTask

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| templateUid | path | True | string |  |
| taskUid | path | True | string |  |
| clientUid | query | True | string |  |

Responses: `200` */* → string.

## /Marti/api/excheck/{checklistUid}/stop

### POST — stopChecklist

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| checklistUid | path | True | string |  |
| clientUid | query | True | string |  |

Responses: `200` */* → string.

## /Marti/api/excheck/{templateUid}/start

### POST — startChecklist

Tags: ex-check-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| templateUid | path | True | string |  |
| clientUid | query | True | string |  |
| callsign | query | True | string |  |
| name | query | True | string |  |
| description | query | True | string |  |
| startTime | query | True | string |  |
| defaultRole | query | False | string |  |

Responses: `200` */* → string.

## /Marti/api/federate-outbound-groups-hop-limit/{federateId}

### GET — getFederateOutboundGroupsHopLimits

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| federateId | path | True | string |  |

Responses: `200` */* → ApiResponseListOutboundGroupHopLimit.

### POST — addFederateOutboundGroupsHopLimit

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| federateId | path | True | string |  |

Request body (required): `application/json` → object.

Responses: `200` */* → ApiResponseString.

### DELETE — removeFederateOutboundGroupsHopLimit

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| federateId | path | True | string |  |
| group | query | True | string |  |

Responses: `200` */* → ApiResponseString.

## /Marti/api/federatecagroups

### POST — addFederateCAGroup

Tags: federation-api.

Request body (required): `application/json` → FederateCAGroupAssociation.

Responses: `200` */* → ApiResponseString.

## /Marti/api/federatecagroups/{caId}

### GET — getFederateCAGroups

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| caId | path | True | string |  |

Responses: `200` */* → ApiResponseListFederateCAGroupAssociation.

### DELETE — removeFederateCAGroup

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| caId | path | True | string |  |
| group | query | True | string |  |
| direction | query | True | string |  |

Responses: `200` */* → ApiResponseString.

## /Marti/api/federatecahops

### POST — setFederateCAHops

Tags: federation-api.

Request body (required): `application/json` → FederateCAHopsAssociation.

Responses: `200` */* → ApiResponseString.

## /Marti/api/federatecertificates

### GET — getFederateCertificates

Tags: federation-api.

Responses: `200` */* → ApiResponseListCertificateSummary.

### POST — saveFederateCertificateCA

Tags: federation-api.

Request body: `application/json` → object.

Responses: `200` */* → ApiResponseString.

## /Marti/api/federatecertificates/{fingerprint}

### DELETE — deleteFederateCertificateCA

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| fingerprint | path | True | string |  |

Responses: `200` */* → ApiResponseString.

## /Marti/api/federatecontacts/{federateId}

### GET — getFederateContacts

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| federateId | path | True | string |  |

Responses: `200` */* → ApiResponseSortedSetRemoteContact.

## /Marti/api/federatedetails

### PUT — updateFederateDetails

Tags: federation-api.

Request body (required): `application/json` → Federate.

Responses: `200` */* → ApiResponseFederate.

## /Marti/api/federatedetails/{federateId}

### DELETE — deleteFederate

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| federateId | path | True | string |  |

Responses: `200` */* → ApiResponseBoolean.

## /Marti/api/federatedetails/{id}

### GET — getFederateDetails

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| id | path | True | string |  |

Responses: `200` */* → ApiResponseFederate.

## /Marti/api/federategroupconfig

### POST — saveFederateGroupConfiguration

Tags: federation-api.

Responses: `200` */* → ApiResponseString.

## /Marti/api/federategroups

### POST — addFederateGroup

Tags: federation-api.

Request body (required): `application/json` → FederateGroupAssociation.

Responses: `200` */* → ApiResponseString.

## /Marti/api/federategroups/{federateId}

### GET — getFederateGroups

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| federateId | path | True | string |  |

Responses: `200` */* → ApiResponseListFederateGroupAssociation.

### DELETE — removeFederateGroup

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| federateId | path | True | string |  |
| group | query | True | string |  |
| direction | query | True | string |  |

Responses: `200` */* → ApiResponseString.

## /Marti/api/federategroupsmap/{federateId}

### GET — getFederateGroupsMap

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| federateId | path | True | string |  |

Responses: `200` */* → ApiResponseMapStringString.

### POST — addFederateGroupMap

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| federateId | path | True | string |  |
| remoteGroup | query | True | string |  |
| localGroup | query | True | string |  |

Responses: `200` */* → ApiResponseString.

### DELETE — removeFederateGroupMap

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| federateId | path | True | string |  |
| remoteGroup | query | True | string |  |
| localGroup | query | True | string |  |

Responses: `200` */* → ApiResponseString.

## /Marti/api/federatemissions/{federateId}

### PUT — updateFederateMissions

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| federateId | path | True | string |  |

Request body (required): `application/json` → FederateMissionPerConnectionSettings.

Responses: `200` */* → ApiResponseFederateMissionPerConnectionSettings.

## /Marti/api/federateremotegroups/{federateId}

### GET — getFederateRemoteGroups

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| federateId | path | True | string |  |

Responses: `200` */* → ApiResponseListString.

## /Marti/api/federates

### GET — getFederates

Tags: federation-api.

Responses: `200` */* → ApiResponseSortedSetFederate.

## /Marti/api/federationconfig

### GET — getFederationConfig

Tags: federation-config-api.

Responses: `200` */* → ApiResponseFederationConfigInfo.

### PUT — modifyFederationConfig

Tags: federation-config-api.

Request body (required): `application/json` → FederationConfigInfo.

Responses: `200` */* → ApiResponseString.

## /Marti/api/federationconfig/verify

### GET — verifyFederationTruststore

Tags: federation-config-api.

Responses: `200` */* → ApiResponseBoolean.

## /Marti/api/fednum

### GET — getNum

Tags: federation-api.

Responses: `200` */* → integer (int32).

## /Marti/api/files/metadata

### GET — getFileMetadata

Tags: file-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| page | query | False | integer (int32) | -1 |
| limit | query | False | integer (int32) | -1 |
| mission | query | False | string | "" |
| missionPackage | query | False | boolean | false |
| name | query | False | string | "" |
| sort | query | False | string | "" |
| ascending | query | False | boolean | true |

Responses: `200` */* → ApiResponseCollectionMapStringString.

## /Marti/api/files/metadata/count

### GET — getFileCount

Tags: file-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| mission | query | False | string | "" |
| missionPackage | query | False | boolean | false |

Responses: `200` */* → ApiResponseInteger.

## /Marti/api/files/{hash}

### GET — getFile_1

Tags: file-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| hash | path | True | string |  |

Responses: `200` */* → string (binary).

### DELETE — deleteFile

Tags: file-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| hash | path | True | string |  |

Responses: `200` OK.

### HEAD — getFileHead

Tags: file-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| hash | path | True | string |  |

Responses: `200` */* → ApiResponseMapStringString.

## /Marti/api/files/{hash}/metadata

### PUT — putMetadata

Tags: file-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| hash | path | True | string |  |
| user | query | False | string | "" |
| expiration | query | False | string | "" |
| keywords | query | False | array[string] | [] |

Responses: `200` OK.

## /Marti/api/groupprefix

### GET — getGroupPrefix

Tags: ldap-api.

Responses: `200` */* → ApiResponseString.

## /Marti/api/groups

### GET — getLdapGroups

Tags: ldap-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| groupNameFilter | query | True | string |  |

Responses: `200` */* → ApiResponseSortedSetLdapGroup.

## /Marti/api/groups/active

### PUT — setActiveGroups_1

Tags: subscription-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| clientUid | query | False | string |  |

Request body (required): `application/json` → array[Group].

Responses: `200` */* → string.

## /Marti/api/groups/activebits

### PUT — setActiveGroups

Tags: subscription-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| clientUid | query | False | string |  |

Request body (required): `application/json` → array[integer (int32)].

Responses: `200` */* → string.

## /Marti/api/groups/all

### GET — getAllGroups

Tags: groups-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| useCache | query | False | boolean | false |
| sendLatestSA | query | False | boolean | false |

Responses: `200` */* → ApiResponseCollectionGroup.

## /Marti/api/groups/groupCacheEnabled

### GET — getGroupCacheEnabled

Tags: groups-api.

Responses: `200` */* → ApiResponseBoolean.

## /Marti/api/groups/members

### GET — getLdapGroupMembers

Tags: ldap-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| groupNameFilter | query | True | array[string] |  |

Responses: `200` */* → ApiResponseInteger.

## /Marti/api/groups/update/{username}

### GET — groupsUpdated

Tags: subscription-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| username | path | True | string |  |

Responses: `200` */* → string.

## /Marti/api/groups/{name}/{direction}

### GET — getGroup

Tags: groups-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| direction | path | True | string |  |

Responses: `200` */* → ApiResponseGroup.

## /Marti/api/home

### GET — getHome

Tags: home-api.

Responses: `200` */* → string.

## /Marti/api/icon/{uid}/{group}/{name}

### GET — getIcon

Tags: iconset-icon-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |
| group | path | True | string |  |
| name | path | True | string |  |

Responses: `200` */* → string (byte).

## /Marti/api/iconimage

### GET — getIconImage

Tags: iconset-icon-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| iconsetpath | query | False | string |  |
| cotType | query | False | string |  |
| medevac | query | False | boolean |  |
| groupName | query | False | string |  |
| role | query | False | string |  |
| color | query | False | integer (int64) |  |
| relative | query | False | boolean |  |

Responses: `200` */* → string (byte).

## /Marti/api/iconset

### POST — postIconsetZip

Tags: iconset-icon-api.

Request body: `application/json` → object.

Responses: `200` */* → ApiResponseString.

## /Marti/api/iconset/all/uid

### GET — getAllIconsetUids

Tags: iconset-icon-api.

Responses: `200` */* → ApiResponseSetString.

## /Marti/api/iconseturl/{uid}

### GET — getAllIconUrlsForIconset

Tags: iconset-icon-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Responses: `200` */* → ApiResponseListString.

## /Marti/api/iconurl

### GET — getIconUrl

Tags: iconset-icon-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| iconsetpath | query | False | string |  |
| cotType | query | False | string |  |
| medevac | query | False | boolean |  |
| groupName | query | False | string |  |
| role | query | False | string |  |
| color | query | False | integer (int64) |  |
| relative | query | False | boolean |  |

Responses: `200` */* → string.

## /Marti/api/injectors/cot/uid

### GET — getAllCotInjectors

Tags: injection-api.

Responses: `200` */* → ApiResponseSetInjectorConfig.

### POST — putCotInjector

Tags: injection-api.

Request body (required): `application/json` → InjectorConfig.

Responses: `200` */* → ApiResponseSetInjectorConfig.

### DELETE — deleteInjector

Tags: injection-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | query | True | string |  |
| toInject | query | True | string |  |

Responses: `200` */* → ApiResponseSetInjectorConfig.

## /Marti/api/injectors/cot/uid/{uid}

### GET — getOneCotInjector

Tags: injection-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Responses: `200` */* → ApiResponseCollectionInjectorConfig.

## /Marti/api/inputs

### GET — getInputMetrics

Tags: submission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| excludeDataFeeds | query | False | boolean | false |

Responses: `200` */* → ApiResponseSortedSetInputMetric.

### POST — createInput

Tags: submission-api.

Request body (required): `application/json` → Input.

Responses: `200` */* → ApiResponseInput.

## /Marti/api/inputs/config

### GET — getConfigInfo

Tags: submission-api.

Responses: `200` */* → ApiResponseMessagingConfigInfo.

### PUT — modifyConfigInfo

Tags: submission-api.

Request body (required): `application/json` → MessagingConfigInfo.

Responses: `200` */* → ApiResponseString.

## /Marti/api/inputs/storeForwardChat/disable

### PUT — disableStoreForwardChat

Tags: submission-api.

Responses: `200` OK.

## /Marti/api/inputs/storeForwardChat/enable

### PUT — enableStoreForwardChat

Tags: submission-api.

Responses: `200` OK.

## /Marti/api/inputs/storeForwardChat/enabled

### GET — isStoreForwardChatEnabled

Tags: submission-api.

Responses: `200` */* → boolean.

## /Marti/api/inputs/{id}

### PUT — modifyInput

Tags: submission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| id | path | True | string |  |

Request body (required): `application/json` → Input.

Responses: `200` */* → ApiResponseConnectionModifyResult.

## /Marti/api/inputs/{name}

### GET — getInputMetric

Tags: submission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → ApiResponseInputMetric.

### DELETE — deoleteInput

Tags: submission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → ApiResponseInput.

## /Marti/api/maplayers

### PUT — updateMapLayer_1

Tags: map-layers-api.

Request body (required): `application/json` → MapLayer.

Responses: `200` */* → ApiResponseMapLayer.

### POST — createMapLayer_1

Tags: map-layers-api.

Request body (required): `application/json` → MapLayer.

Responses: `200` */* → ApiResponseMapLayer.

## /Marti/api/maplayers/all

### GET — getAllMapLayers

Tags: map-layers-api.

Responses: `200` */* → ApiResponseCollectionMapLayer.

## /Marti/api/maplayers/{uid}

### GET — getMapLayerForUid

Tags: map-layers-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Responses: `200` */* → ApiResponseMapLayer.

### DELETE — deleteMapLayer_1

Tags: map-layers-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Responses: `200` OK.

## /Marti/api/missioncount

### GET — countAllMissions

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| passwordProtected | query | False | boolean | true |
| defaultRole | query | False | boolean | true |
| tool | query | False | string |  |

Responses: `200` */* → ApiResponseInteger.

## /Marti/api/missions

### GET — getAllMissions

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| passwordProtected | query | False | boolean | false |
| defaultRole | query | False | boolean | false |
| tool | query | False | string |  |

Responses: `200` */* → ApiResponseListMission.

### DELETE — deleteMissionByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| guid | query | True | string |  |
| creatorUid | query | False | string | "" |
| deepDelete | query | False | boolean | false |

Responses: `200` */* → ApiResponseSetMission.

## /Marti/api/missions/all/invitations

### GET — getAllMissionInvitations

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| clientUid | query | False | string | "" |

Responses: `200` */* → ApiResponseSetString.

## /Marti/api/missions/all/logs

### GET — getAllLogEntries

Tags: mission-api.

Responses: `200` */* → ApiResponseListLogEntry.

## /Marti/api/missions/all/subscriptions

### GET — getAllMissionSubscriptions

Tags: mission-api.

Responses: `200` */* → ApiResponseListEntryStringString.

## /Marti/api/missions/all/subscriptions/guid

### GET — getAllMissionSubscriptionsWithGuid

Tags: mission-api.

Responses: `200` */* → ApiResponseListEntryEntryStringStringString.

## /Marti/api/missions/guid/{childGuid}/parent

### DELETE — clearParentByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| childGuid | path | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/guid/{childGuid}/parent/guid/{parentGuid}

### PUT — setParentByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| childGuid | path | True | string |  |
| parentGuid | path | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/guid/{guid}

### GET — getMissionByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| guid | path | True | string |  |
| password | query | False | string | "" |
| changes | query | False | boolean | false |
| logs | query | False | boolean | false |
| secago | query | False | integer (int64) |  |
| start | query | False | string (date-time) |  |
| end | query | False | string (date-time) |  |

Responses: `200` */* → ApiResponseSetMission.

## /Marti/api/missions/guid/{guid}/children

### GET — getChildrenByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| guid | path | True | string |  |

Responses: `200` */* → ApiResponseSetMission.

## /Marti/api/missions/guid/{guid}/contents

### PUT — addMissionContentByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| guid | path | True | string |  |
| creatorUid | query | False | string | "" |

Request body (required): `application/json` → MissionContent.

Responses: `200` */* → ApiResponseSetMission.

### DELETE — removeMissionContentByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| guid | path | True | string |  |
| hash | query | False | string |  |
| uid | query | False | string |  |
| creatorUid | query | False | string | "" |

Responses: `200` */* → ApiResponseSetMission.

## /Marti/api/missions/guid/{guid}/expiration

### PUT — setExpirationByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| guid | path | True | string |  |
| expiration | query | False | integer (int64) |  |

Responses: `200` OK.

## /Marti/api/missions/guid/{guid}/externaldata

### POST — setExternalMissionDataByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| guid | path | True | string |  |
| creatorUid | query | True | string |  |

Request body (required): `application/json` → ExternalMissionData.

Responses: `201` */* → ApiResponseExternalMissionData.

## /Marti/api/missions/guid/{guid}/externaldata/{id}

### DELETE — deleteExternalMissionDataByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| guid | path | True | string |  |
| id | path | True | string |  |
| notes | query | True | string |  |
| creatorUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/guid/{guid}/externaldata/{id}/change

### POST — notifyExternalDataChangedByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| guid | path | True | string |  |
| id | path | True | string |  |
| creatorUid | query | True | string |  |
| notes | query | True | string |  |

Request body (required): `application/json` → string.

Responses: `200` OK.

## /Marti/api/missions/guid/{guid}/password

### PUT — setPasswordByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| guid | path | True | string |  |
| password | query | False | string | "" |
| creatorUid | query | False | string | "" |

Responses: `200` OK.

### DELETE — removePasswordByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| guid | path | True | string |  |
| creatorUid | query | False | string | "" |

Responses: `200` OK.

## /Marti/api/missions/guid/{guid}/send

### POST — sendMissionArchiveByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| guid | path | True | string |  |

Responses: `200` */* → ApiResponseSetMission.

## /Marti/api/missions/guid/{missionGuid}/contacts

### GET — getMissionContactsByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |

Responses: `200` */* → array[RemoteSubscription].

## /Marti/api/missions/guid/{missionGuid}/cot

### GET — getLatestMissionCotEventsByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| path | query | False | string |  |

Responses: `200` */* → string.

## /Marti/api/missions/guid/{missionGuid}/invitations

### GET — getMissionInvitationsByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |

Responses: `200` */* → ApiResponseListMissionInvitation.

## /Marti/api/missions/guid/{missionGuid}/invite

### POST — sendMissionInvitesByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| creatorUid | query | False | string |  |

Responses: `200` OK.

## /Marti/api/missions/guid/{missionGuid}/invite/{type}/{invitee}

### PUT — inviteToMissionByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| type | path | True | string |  |
| invitee | path | True | string |  |
| creatorUid | query | True | string |  |
| role | query | False | string |  |

Responses: `200` OK.

### DELETE — uninviteFromMissionByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| type | path | True | string |  |
| invitee | path | True | string |  |
| creatorUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/guid/{missionGuid}/kml

### GET — getKmlByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| download | query | False | boolean | false |

Responses: `200` */* → string.

## /Marti/api/missions/guid/{missionGuid}/layers

### GET — getMissionLayersByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |

Responses: `200` */* → ApiResponseListMissionLayer.

### PUT — createMissionLayerByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| name | query | True | string |  |
| type | query | True | string |  |
| uid | query | False | string |  |
| parentUid | query | False | string |  |
| afterUid | query | False | string |  |
| creatorUid | query | True | string |  |

Responses: `200` */* → ApiResponseMissionLayer.

### DELETE — deleteMissionLayerByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| uid | query | True | array[string] |  |
| creatorUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/guid/{missionGuid}/layers/parent

### PUT — setLayerParentByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| layerUid | query | True | array[string] |  |
| parentUid | query | False | string |  |
| afterUid | query | False | string |  |
| creatorUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/guid/{missionGuid}/layers/{layerUid}

### GET — getMissionLayerByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| layerUid | path | True | string |  |

Responses: `200` */* → ApiResponseMissionLayer.

## /Marti/api/missions/guid/{missionGuid}/layers/{layerUid}/name

### PUT — setLayerNameByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| layerUid | path | True | string |  |
| name | query | True | string |  |
| creatorUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/guid/{missionGuid}/layers/{layerUid}/position

### PUT — setLayerPositionByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| layerUid | path | True | string |  |
| afterUid | query | False | string |  |
| creatorUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/guid/{missionGuid}/maplayers

### PUT — updateMapLayerByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| creatorUid | query | True | string |  |

Request body (required): `application/json` → MapLayer.

Responses: `200` */* → ApiResponseMapLayer.

### POST — createMapLayerByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| creatorUid | query | True | string |  |

Request body (required): `application/json` → MapLayer.

Responses: `200` */* → ApiResponseMapLayer.

## /Marti/api/missions/guid/{missionGuid}/role

### GET — getMissionRoleFromTokenByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |

Responses: `200` */* → ApiResponseMissionRole.

### PUT — setMissionRoleByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| clientUid | query | False | string | "" |
| username | query | False | string | "" |
| role | query | False | string |  |

Responses: `200` OK.

## /Marti/api/missions/guid/{missionGuid}/subscription

### GET — getSubscriptionForUserByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| uid | query | False | string | "" |

Responses: `200` */* → ApiResponseMissionSubscription.

### PUT — createMissionSubscriptionByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | query | False | string | "" |
| topic | query | False | string | "" |
| password | query | False | string | "" |
| secago | query | False | integer (int64) |  |
| start | query | False | string (date-time) |  |
| end | query | False | string (date-time) |  |
| missionGuid | path | True | string |  |

Responses: `201` */* → ApiResponseMissionSubscription.

### POST — setSubscriptionRoleByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| creatorUid | query | True | string |  |

Request body (required): `application/json` → array[MissionSubscription].

Responses: `200` OK.

### DELETE — deleteMissionSubscriptionByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | query | False | string | "" |
| topic | query | False | string | "" |
| disconnectOnly | query | False | boolean | true |
| missionGuid | path | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/guid/{missionGuid}/subscriptions

### GET — getMissionSubscriptionsByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |

Responses: `200` */* → ApiResponseListString.

## /Marti/api/missions/guid/{missionGuid}/subscriptions/roles

### GET — getMissionSubscriptionRolesByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |

Responses: `200` */* → ApiResponseListMissionSubscription.

## /Marti/api/missions/guid/{missionName}/layers/{layerUid}/position

### PUT — setLayerPosition

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| layerUid | path | True | string |  |
| afterUid | query | False | string |  |
| creatorUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/invitations

### GET — getAllMissionInvitationsWithPasswords

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| clientUid | query | True | string |  |

Responses: `200` */* → ApiResponseSetMissionInvitation.

## /Marti/api/missions/logs/entries

### PUT — updateLogEntry

Tags: mission-api.

Request body (required): `application/json` → LogEntry.

Responses: `201` */* → ApiResponseLogEntry.

### POST — postLogEntry

Tags: mission-api.

Request body (required): `application/json` → LogEntry.

Responses: `201` */* → ApiResponseLogEntry.

## /Marti/api/missions/logs/entries/{id}

### GET — getOneLogEntry

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| id | path | True | string |  |

Responses: `200` */* → ApiResponseListLogEntry.

### DELETE — deleteLogEntry

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| id | path | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/{childName}/parent

### DELETE — clearParent

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| childName | path | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/{childName}/parent/{parentName}

### PUT — setParent

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| childName | path | True | string |  |
| parentName | path | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/{missionGuid}/feed

### POST — addFeedByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| creatorUid | query | True | string |  |
| dataFeedUid | query | True | string |  |
| filterPolygon | query | False | array[string] |  |
| filterCotTypes | query | False | string |  |
| filterCallsign | query | False | string |  |

Responses: `200` OK.

## /Marti/api/missions/{missionGuid}/maplayers/{uid}

### DELETE — deleteMapLayerByGuid

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionGuid | path | True | string |  |
| creatorUid | query | True | string |  |
| uid | path | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/{missionName}/copy

### PUT — copyMission

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| creatorUid | query | True | string |  |
| copyName | query | True | string |  |
| copyPath | query | False | string |  |
| defaultRole | query | False | string |  |
| password | query | False | string |  |

Responses: `200` */* → ApiResponseSetMission.

## /Marti/api/missions/{missionName}/feed

### POST — addFeed

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| creatorUid | query | True | string |  |
| dataFeedUid | query | True | string |  |
| filterPolygon | query | False | array[string] |  |
| filterCotTypes | query | False | string |  |
| filterCallsign | query | False | string |  |

Responses: `200` OK.

## /Marti/api/missions/{missionName}/feed/{uid}

### DELETE — removeFeed

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| uid | path | True | string |  |
| creatorUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/{missionName}/invitations

### GET — getMissionInvitations

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |

Responses: `200` */* → ApiResponseListMissionInvitation.

## /Marti/api/missions/{missionName}/layers

### GET — getMissionLayers

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |

Responses: `200` */* → ApiResponseListMissionLayer.

### PUT — createMissionLayer

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| name | query | True | string |  |
| type | query | True | string |  |
| uid | query | False | string |  |
| parentUid | query | False | string |  |
| afterUid | query | False | string |  |
| creatorUid | query | True | string |  |

Responses: `200` */* → ApiResponseMissionLayer.

### DELETE — deleteMissionLayer

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| uid | query | True | array[string] |  |
| creatorUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/{missionName}/layers/parent

### PUT — setLayerParent

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| layerUid | query | True | array[string] |  |
| parentUid | query | False | string |  |
| afterUid | query | False | string |  |
| creatorUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/{missionName}/layers/{layerUid}

### GET — getMissionLayer

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| layerUid | path | True | string |  |

Responses: `200` */* → ApiResponseMissionLayer.

## /Marti/api/missions/{missionName}/layers/{layerUid}/name

### PUT — setLayerName

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| layerUid | path | True | string |  |
| name | query | True | string |  |
| creatorUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/{missionName}/log

### GET — getLogEntry

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| secago | query | False | integer (int64) |  |
| start | query | False | string (date-time) |  |
| end | query | False | string (date-time) |  |

Responses: `200` */* → ApiResponseListLogEntry.

## /Marti/api/missions/{missionName}/maplayers

### PUT — updateMapLayer

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| creatorUid | query | True | string |  |

Request body (required): `application/json` → MapLayer.

Responses: `200` */* → ApiResponseMapLayer.

### POST — createMapLayer

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| creatorUid | query | True | string |  |

Request body (required): `application/json` → MapLayer.

Responses: `200` */* → ApiResponseMapLayer.

## /Marti/api/missions/{missionName}/maplayers/{uid}

### DELETE — deleteMapLayer

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| creatorUid | query | True | string |  |
| uid | path | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/{missionName}/role

### GET — getMissionRoleFromToken

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |

Responses: `200` */* → ApiResponseMissionRole.

### PUT — setMissionRole

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| clientUid | query | False | string | "" |
| username | query | False | string | "" |
| role | query | False | string |  |

Responses: `200` OK.

## /Marti/api/missions/{missionName}/subscription

### GET — getSubscriptionForUser

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| uid | query | False | string | "" |

Responses: `200` */* → ApiResponseMissionSubscription.

### PUT — createMissionSubscription

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | query | False | string | "" |
| topic | query | False | string | "" |
| password | query | False | string | "" |
| secago | query | False | integer (int64) |  |
| start | query | False | string (date-time) |  |
| end | query | False | string (date-time) |  |
| missionName | path | True | string |  |

Responses: `201` */* → ApiResponseMissionSubscription.

### POST — setSubscriptionRole

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |
| creatorUid | query | True | string |  |

Request body (required): `application/json` → array[MissionSubscription].

Responses: `200` OK.

### DELETE — deleteMissionSubscription

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | query | False | string | "" |
| topic | query | False | string | "" |
| disconnectOnly | query | False | boolean | true |
| missionName | path | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/{missionName}/subscriptions

### GET — getMissionSubscriptions

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |

Responses: `200` */* → ApiResponseListString.

## /Marti/api/missions/{missionName}/subscriptions/roles

### GET — getMissionSubscriptionRoles

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| missionName | path | True | string |  |

Responses: `200` */* → ApiResponseListMissionSubscription.

## /Marti/api/missions/{missionName}/token

### GET — getReadOnlyAccessToken

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| password | query | False | string | "" |
| missionName | path | True | string |  |

Responses: `201` */* → ApiResponseString.

## /Marti/api/missions/{name}

### GET — getMission

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| password | query | False | string | "" |
| changes | query | False | boolean | false |
| logs | query | False | boolean | false |
| secago | query | False | integer (int64) |  |
| start | query | False | string (date-time) |  |
| end | query | False | string (date-time) |  |

Responses: `200` */* → ApiResponseSetMission.

### PUT — createMission

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| creatorUid | query | False | string | "" |
| group | query | False | array[string] | ["__ANON__"] |
| description | query | False | string | "" |
| chatRoom | query | False | string | "" |
| baseLayer | query | False | string | "" |
| bbox | query | False | string | "" |
| boundingPolygon | query | False | array[string] | [] |
| path | query | False | string | "" |
| classification | query | False | string | "" |
| tool | query | False | string | "public" |
| password | query | False | string |  |
| defaultRole | query | False | string |  |
| expiration | query | False | integer (int64) |  |
| inviteOnly | query | False | boolean | false |
| allowGroupChange | query | False | boolean | false |

Request body: `application/json` → string (byte).

Responses: `200` */* → ApiResponseSetMission.

### POST — createMissionAllowDupe

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| creatorUid | query | False | string | "" |
| group | query | False | array[string] | ["__ANON__"] |
| description | query | False | string | "" |
| chatRoom | query | False | string | "" |
| baseLayer | query | False | string | "" |
| bbox | query | False | string | "" |
| boundingPolygon | query | False | array[string] | [] |
| path | query | False | string | "" |
| classification | query | False | string | "" |
| tool | query | False | string | "public" |
| password | query | False | string |  |
| defaultRole | query | False | string |  |
| expiration | query | False | integer (int64) |  |
| inviteOnly | query | False | boolean | false |
| allowDupe | query | False | boolean | false |

Request body: `application/json` → string (byte).

Responses: `200` */* → ApiResponseSetMission.

### DELETE — deleteMission

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| creatorUid | query | False | string | "" |
| deepDelete | query | False | boolean | false |

Responses: `200` */* → ApiResponseSetMission.

## /Marti/api/missions/{name}/archive

### GET — getMissionArchive_1

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → string (byte).

## /Marti/api/missions/{name}/changes

### GET — getMissionChanges

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| secago | query | False | integer (int64) |  |
| start | query | False | string (date-time) |  |
| end | query | False | string (date-time) |  |
| squashed | query | False | boolean | true |

Responses: `200` */* → ApiResponseSetMissionChange.

## /Marti/api/missions/{name}/children

### GET — getChildren

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → ApiResponseSetMission.

## /Marti/api/missions/{name}/contacts

### GET — getMissionContacts

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → array[RemoteSubscription].

## /Marti/api/missions/{name}/content/{hash}/keywords

### PUT — addContentKeyword

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| hash | path | True | string |  |
| creatorUid | query | False | string | "" |

Request body (required): `application/json` → array[string].

Responses: `200` OK.

### DELETE — clearContentKeywords

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| hash | path | True | string |  |
| creatorUid | query | False | string | "" |

Responses: `200` OK.

## /Marti/api/missions/{name}/contents

### PUT — addMissionContent

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| creatorUid | query | False | string | "" |

Request body (required): `application/json` → MissionContent.

Responses: `200` */* → ApiResponseSetMission.

### DELETE — removeMissionContent

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| hash | query | False | string |  |
| uid | query | False | string |  |
| creatorUid | query | False | string | "" |

Responses: `200` */* → ApiResponseSetMission.

## /Marti/api/missions/{name}/contents/missionpackage

### PUT — addMissionPackage

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| creatorUid | query | True | string |  |

Request body (required): `application/json` → string (byte).

Responses: `200` */* → ApiResponseListMissionChange.

## /Marti/api/missions/{name}/cot

### GET — getLatestMissionCotEvents

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| path | query | False | string |  |

Responses: `200` */* → string.

## /Marti/api/missions/{name}/expiration

### PUT — setExpiration_1

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| expiration | query | False | integer (int64) |  |

Responses: `200` OK.

## /Marti/api/missions/{name}/externaldata

### POST — setExternalMissionData

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| creatorUid | query | True | string |  |

Request body (required): `application/json` → ExternalMissionData.

Responses: `201` */* → ApiResponseExternalMissionData.

## /Marti/api/missions/{name}/externaldata/{id}

### DELETE — deleteExternalMissionData

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| id | path | True | string |  |
| notes | query | True | string |  |
| creatorUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/{name}/externaldata/{id}/change

### POST — notifyExternalDataChanged

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| id | path | True | string |  |
| creatorUid | query | True | string |  |
| notes | query | True | string |  |

Request body (required): `application/json` → string.

Responses: `200` OK.

## /Marti/api/missions/{name}/invite

### POST — sendMissionInvites

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| creatorUid | query | False | string |  |

Responses: `200` OK.

## /Marti/api/missions/{name}/invite/{type}/{invitee}

### PUT — inviteToMission

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| type | path | True | string |  |
| invitee | path | True | string |  |
| creatorUid | query | True | string |  |
| role | query | False | string |  |

Responses: `200` OK.

### DELETE — uninviteFromMission

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| type | path | True | string |  |
| invitee | path | True | string |  |
| creatorUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/missions/{name}/keywords

### PUT — setKeywords

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| creatorUid | query | False | string | "" |

Request body (required): `application/json` → array[string].

Responses: `200` */* → ApiResponseSetMission.

### DELETE — clearKeywords

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| creatorUid | query | False | string | "" |

Responses: `200` */* → ApiResponseSetMission.

## /Marti/api/missions/{name}/keywords/{keyword}

### DELETE — removeKeyword

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| keyword | path | True | string |  |
| creatorUid | query | False | string | "" |

Responses: `200` */* → ApiResponseSetMission.

## /Marti/api/missions/{name}/kml

### GET — getKml

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| download | query | False | boolean | false |

Responses: `200` */* → string.

## /Marti/api/missions/{name}/parent

### GET — getParent

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → ApiResponseMission.

## /Marti/api/missions/{name}/password

### PUT — setPassword

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| password | query | False | string | "" |
| creatorUid | query | False | string | "" |

Responses: `200` OK.

### DELETE — removePassword

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| creatorUid | query | False | string | "" |

Responses: `200` OK.

## /Marti/api/missions/{name}/send

### POST — sendMissionArchive

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → ApiResponseSetMission.

## /Marti/api/missions/{name}/uid/{uid}/keywords

### PUT — addUidKeyword

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| uid | path | True | string |  |
| creatorUid | query | False | string | "" |

Request body (required): `application/json` → array[string].

Responses: `200` OK.

### DELETE — clearUidKeywords

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| uid | path | True | string |  |
| creatorUid | query | False | string | "" |

Responses: `200` OK.

## /Marti/api/node/id

### GET — getNodeId

Tags: version-api.

Responses: `200` */* → string.

## /Marti/api/outgoingconnections

### GET — getOutgoingConnections

Tags: federation-api.

Responses: `200` */* → ApiResponseSortedSetOutgoingConnectionSummary.

### PUT — updateOutgoingConnection

Tags: federation-api.

Request body (required): `application/json` → object.

Responses: `200` */* → ApiResponseFederationOutgoing.

### POST — createOutgoingConnection

Tags: federation-api.

Request body (required): `application/json` → FederationOutgoing.

Responses: `200` */* → ApiResponseFederationOutgoing.

## /Marti/api/outgoingconnections/{name}

### GET — getOutgoingConnection

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → ApiResponseFederationOutgoing.

### DELETE — deleteOutgoingConnection

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → ApiResponseBoolean.

## /Marti/api/outgoingconnectionstatus/{name}

### GET — getConnectionStatus

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |

Responses: `200` */* → ApiResponseConnectionStatus.

### POST — changeConnectionStatus

Tags: federation-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| newStatus | query | True | boolean |  |

Responses: `200` */* → ApiResponseBoolean.

## /Marti/api/pagedmissions

### GET — getPagedMissions

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| passwordProtected | query | False | boolean | true |
| defaultRole | query | False | boolean | true |
| page | query | False | integer (int32) | 0 |
| pagesize | query | False | integer (int32) | 10 |
| tool | query | False | string |  |
| sort | query | False | string | "" |
| nameFilter | query | False | string | "" |
| uidFilter | query | False | string | "" |
| ascending | query | False | boolean | true |

Responses: `200` */* → ApiResponseListMission.

## /Marti/api/plugins/info/all

### GET — getAllPluginInfo

Tags: plugin-manager-api.

Responses: `200` */* → ApiResponseCollectionPluginInfo.

## /Marti/api/plugins/info/all/started

### POST — changeAllPluginStartedStatus

Tags: plugin-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| status | query | True | boolean |  |

Responses: `200` */* → ApiResponseBoolean.

## /Marti/api/plugins/info/archive

### POST — changePluginArchiveSetting

Tags: plugin-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | query | True | string |  |
| archiveEnabled | query | True | boolean |  |

Responses: `200` */* → ApiResponseBoolean.

## /Marti/api/plugins/info/enabled

### POST — changePluginEnabledSetting

Tags: plugin-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | query | True | string |  |
| status | query | True | boolean |  |

Responses: `200` */* → ApiResponseBoolean.

## /Marti/api/plugins/info/started

### POST — changePluginStartedStatus

Tags: plugin-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | query | True | string |  |
| status | query | True | boolean |  |

Responses: `200` */* → ApiResponseBoolean.

## /Marti/api/plugins/{name}/submit

### GET — requestFromPlugin

Tags: plugin-data-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| allRequestParams | query | True | object |  |

Responses: `200` */* → string.

### PUT — submitToPluginUTF8

Tags: plugin-data-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| allRequestParams | query | True | object |  |

Request body (required): `application/json` → string (byte).

Responses: `200` */* → ApiResponseString.

### POST — updateInPlugin

Tags: plugin-data-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| allRequestParams | query | True | object |  |

Request body (required): `application/json` → string (byte).

Responses: `200` */* → ApiResponseString.

### DELETE — deleteFromPlugin

Tags: plugin-data-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| allRequestParams | query | True | object |  |

Responses: `200` */* → ApiResponseString.

## /Marti/api/plugins/{name}/submit/result

### PUT — submitToPluginUTF8WithResult

Tags: plugin-data-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| allRequestParams | query | True | object |  |

Request body (required): `application/json` → string (byte).

Responses: `200` */* → string.

## /Marti/api/properties/uids

### GET — getAllPropertyKeys

Tags: properties-api.

Responses: `200` */* → ApiResponseCollectionString.

## /Marti/api/properties/{uid}

### PUT — storeProperty

Tags: properties-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Request body (required): `application/json` → object.

Responses: `200` */* → ApiResponseEntryStringString.

## /Marti/api/properties/{uid}/all

### GET — getAllPropertyForUid

Tags: properties-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Responses: `200` */* → ApiResponseMapStringCollectionString.

### DELETE — clearAllProperty

Tags: properties-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Responses: `200` OK.

## /Marti/api/properties/{uid}/{key}

### GET — getPropertyForUid

Tags: properties-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |
| key | path | True | string |  |

Responses: `200` */* → ApiResponseCollectionString.

### DELETE — clearProperty

Tags: properties-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |
| key | path | True | string |  |

Responses: `200` OK.

## /Marti/api/qos/conf

### GET — getQosConf

Tags: qo-s-api.

Responses: `200` */* → ApiResponseQos.

## /Marti/api/qos/delivery/enable

### PUT — enableDelivery

Tags: qo-s-api.

Request body (required): `application/json` → boolean.

Responses: `200` OK.

## /Marti/api/qos/delivery/set

### PUT — setDelivery

Tags: qo-s-api.

Request body (required): `application/json` → array[RateLimitRule].

Responses: `200` OK.

## /Marti/api/qos/dos/enable

### PUT — enableDOS

Tags: qo-s-api.

Request body (required): `application/json` → boolean.

Responses: `200` OK.

## /Marti/api/qos/ratelimit/delivery/active

### GET — getActiveDeliveryRateLimit

Tags: qo-s-api.

Responses: `200` */* → ApiResponseEntryIntegerInteger.

## /Marti/api/qos/ratelimit/dos/active

### GET — getActiveDOSRateLimit

Tags: qo-s-api.

Responses: `200` */* → ApiResponseEntryIntegerInteger.

## /Marti/api/qos/ratelimit/read/active

### GET — getActiveReadRateLimit

Tags: qo-s-api.

Responses: `200` */* → ApiResponseEntryIntegerInteger.

## /Marti/api/qos/read/enable

### PUT — enableRead

Tags: qo-s-api.

Request body (required): `application/json` → boolean.

Responses: `200` OK.

## /Marti/api/qos/read/set

### PUT — setRead

Tags: qo-s-api.

Request body (required): `application/json` → array[RateLimitRule].

Responses: `200` OK.

## /Marti/api/repeater/list

### GET — getList

Tags: repeater-api.

Responses: `200` */* → ApiResponseListRepeatable.

## /Marti/api/repeater/period

### GET — getPeriod

Tags: repeater-api.

Responses: `200` */* → ApiResponseInteger.

### POST — setPeriod

Tags: repeater-api.

Request body (required): `application/json` → integer (int32).

Responses: `200` OK.

## /Marti/api/repeater/remove/{uid}

### GET — remove

Tags: repeater-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Responses: `200` */* → ApiResponseBoolean.

## /Marti/api/resources/{hash}

### GET — getResource

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| hash | path | True | string |  |

Responses: `200` */* → ApiResponseListResource.

## /Marti/api/retention/mission/{name}/expiry/{time}

### PUT — scheduleMissionExpiration

Tags: retention-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| time | path | True | integer (int64) |  |

Responses: `200` OK.

## /Marti/api/retention/missionarchive

### GET — getMissionArchive

Tags: retention-api.

Responses: `200` */* → ApiResponseString.

## /Marti/api/retention/missionarchiveconfig

### GET — getMissionArchiveConfig

Tags: retention-api.

Responses: `200` */* → ApiResponseMissionArchiveConfig.

### PUT — updateMissionArchiveConfig

Tags: retention-api.

Request body (required): `application/json` → MissionArchiveConfig.

Responses: `200` */* → ApiResponseMissionArchiveConfig.

## /Marti/api/retention/policy

### GET — getRetentionPolicy

Tags: retention-api.

Responses: `200` */* → ApiResponseMapStringInteger.

### PUT — updateRetentionPolicy

Tags: retention-api.

Request body (required): `application/json` → object.

Responses: `200` */* → ApiResponseMapStringInteger.

## /Marti/api/retention/resource/{name}/expiry/{time}

### PUT — scheduleResourceExpiration

Tags: retention-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| name | path | True | string |  |
| time | path | True | integer (int64) |  |

Responses: `200` OK.

## /Marti/api/retention/restoremission

### POST — restoreMissionFromArchive

Tags: retention-api.

Request body (required): `application/json` → integer (int32).

Responses: `200` */* → ApiResponseString.

## /Marti/api/retention/service/schedule

### GET — getRetentionServiceSchedule

Tags: retention-api.

Responses: `200` */* → ApiResponseString.

### PUT — setRetentionServiceSchedule

Tags: retention-api.

Request body (required): `application/json` → string.

Responses: `200` */* → ApiResponseString.

## /Marti/api/security/config

### GET — getSecConfig

Tags: security-authentication-api.

Responses: `200` */* → ApiResponseSecurityConfigInfo.

### PUT — modifySecConfig

Tags: security-authentication-api.

Request body (required): `application/json` → SecurityConfigInfo.

Responses: `200` */* → ApiResponseString.

## /Marti/api/security/isSecure

### GET — isSecure

Tags: security-authentication-api.

Responses: `200` */* → ApiResponseString.

## /Marti/api/security/verifyConfig

### GET — verifyConfig

Tags: security-authentication-api.

Responses: `200` */* → ApiResponseString.

## /Marti/api/subscriptions/add

### POST — addSubscription

Tags: subscription-api.

Request body (required): `application/json` → tmpStaticSub.

Responses: `200` */* → ApiResponseSubscriptionInfo.

## /Marti/api/subscriptions/all

### GET — getAllSubscriptions

Tags: subscription-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| sortBy | query | False | string | "CALLSIGN" |
| direction | query | False | string | "ASCENDING" |
| page | query | False | integer (int32) | -1 |
| limit | query | False | integer (int32) | -1 |

Responses: `200` */* → ApiResponseSetSubscriptionInfo.

## /Marti/api/subscriptions/delete/{uid}

### DELETE — deleteSubscription

Tags: subscription-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Responses: `200` */* → ApiResponseString.

## /Marti/api/subscriptions/incognito/{uid}

### POST — toggleIncognito

Tags: subscription-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Responses: `200` */* → string.

## /Marti/api/subscriptions/{clientUid}/filter

### PUT — setFilter

Tags: subscription-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| clientUid | path | True | string |  |

Request body (required): `application/xml` → Filter.

Responses: `200` */* → string.

### DELETE — deleteFilter

Tags: subscription-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| clientUid | path | True | string |  |

Responses: `200` */* → string.

## /Marti/api/sync/metadata/{hash}/expiration

### PUT — setExpiration

Tags: metadata-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| hash | path | True | string |  |
| expiration | query | True | integer (int64) |  |

Responses: `200` */* → string.

## /Marti/api/sync/metadata/{hash}/keywords

### PUT — setMetadataKeywords

Tags: metadata-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| hash | path | True | string |  |

Request body (required): `application/json` → array[string].

Responses: `200` */* → string.

## /Marti/api/sync/metadata/{hash}/{metadata}

### PUT — setMetadata

Tags: metadata-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| hash | path | True | string |  |
| metadata | path | True | string |  |

Request body (required): `application/json` → string.

Responses: `200` */* → string.

## /Marti/api/sync/search

### GET — searchSync

Tags: mission-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| box | query | False | string |  |
| circle | query | False | string |  |
| startTime | query | False | string (date-time) |  |
| endTime | query | False | string (date-time) |  |
| minAltitude | query | False | number (double) |  |
| maxAltitude | query | False | number (double) |  |
| filename | query | False | string |  |
| keyword | query | False | array[string] |  |
| mimetype | query | False | string |  |
| name | query | False | string |  |
| uid | query | False | string |  |
| hash | query | False | string |  |
| mission | query | False | string |  |
| tool | query | False | string |  |

Responses: `200` */* → ApiResponseNavigableSetResource.

## /Marti/api/sync/sequence/{key}

### GET — getNextInSequence

Tags: sequence-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| key | path | True | string |  |

Responses: `200` */* → string.

## /Marti/api/tls/config

### GET — getConfig

Tags: cert-manager-api.

Responses: `200` */* → string.

## /Marti/api/tls/makeClientKeyStore

### GET — makeKeyStore

Tags: cert-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| cn | query | False | string |  |
| password | query | False | string | "atakatak" |

Responses: `200` */* → string (byte).

## /Marti/api/tls/profile/enrollment

### GET — getEnrollmentTimeProfiles

Tags: profile-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| clientUid | query | True | string |  |

Responses: `200` */* → string (byte).

## /Marti/api/tls/profile/tool/{toolName}/file

### GET — tlsGetProfileDirectoryContent

Tags: profile-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| toolName | path | True | string |  |
| relativePath | query | True | array[string] |  |
| syncSecago | query | False | integer (int64) | -1 |
| clientUid | query | True | string |  |

Responses: `200` OK.

## /Marti/api/tls/signClient

### POST — signClientCert

Tags: cert-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| clientUid | query | False | string | "" |
| version | query | False | string |  |

Request body (required): `application/json` → string.

Responses: `200` */* → string (byte).

## /Marti/api/tls/signClient/v2

### POST — signClientCertV2

Tags: cert-manager-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| clientUid | query | False | string | "" |
| version | query | False | string |  |

Request body (required): `application/json` → string.

Responses: `200` */* → string.

## /Marti/api/token

### GET — getAll

Tags: token-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| expired | query | False | boolean | false |

Responses: `200` */* → ApiResponseListTokenResult.

## /Marti/api/token/revoke/{tokens}

### DELETE — revokeTokens

Tags: token-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| tokens | path | True | string |  |

Responses: `200` OK.

## /Marti/api/token/{token}

### DELETE — revokeToken

Tags: token-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| token | path | True | string |  |

Responses: `200` OK.

## /Marti/api/uidsearch

### GET — getUIDResults

Tags: uid-search-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| startDate | query | True | string |  |
| endDate | query | True | string |  |

Responses: `200` */* → ApiResponseListUIDResult.

## /Marti/api/users/all

### GET — getAllUsers_2

Tags: groups-api.

Responses: `200` */* → ApiResponseSortedSetUser.

## /Marti/api/users/{connectionId}

### GET — getUser

Tags: groups-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| connectionId | path | True | string |  |

Responses: `200` */* → ApiResponseUserGroups.

## /Marti/api/util/isAdmin

### GET — isAdmin

Tags: home-api.

Responses: `200` */* → boolean.

## /Marti/api/util/user/roles

### GET — getUserRoles

Tags: home-api.

Responses: `200` */* → array[string].

## /Marti/api/ver

### GET — getVer

Tags: home-api.

Responses: `200` */* → string.

## /Marti/api/version

### GET — getVersion

Tags: version-api.

Responses: `200` */* → string.

## /Marti/api/version/config

### GET — getVersionConfig

Tags: version-api.

Responses: `200` */* → ApiResponseServerConfig.

## /Marti/api/version/info

### GET — getVersionInfo

Tags: version-api.

Responses: `200` */* → VersionInfo.

## /Marti/api/video

### GET — getVideoCollections

Tags: video-connection-manager-v-2.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| protocol | query | False | string |  |

Responses: `200` */* → VideoCollections.

### POST — createVideoConnection

Tags: video-connection-manager-v-2.

Request body (required): `application/json` → VideoCollections.

Responses: `200` OK.

## /Marti/api/video/{uid}

### GET — getVideoConnection

Tags: video-connection-manager-v-2.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Responses: `200` */* → VideoConnection.

### PUT — updateVideoConnection

Tags: video-connection-manager-v-2.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Request body (required): `application/json` → VideoConnection.

Responses: `200` OK.

### DELETE — deleteVideoConnection

Tags: video-connection-manager-v-2.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |

Responses: `200` OK.

## /Marti/api/xmpp/transfer/{uid}/{filename}

### GET — getFile

Tags: xmpp-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |
| filename | path | True | string |  |

Responses: `200` */* → string.

### PUT — putFile

Tags: xmpp-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| uid | path | True | string |  |
| filename | path | True | string |  |

Request body (required): `application/json` → string (byte).

Responses: `200` */* → string.

## /files/api/config

### GET — getFileConfiguration

Tags: file-configuration-api.

Responses: `200` */* → FileConfigurationModel.

### POST — setFileConfiguration

Tags: file-configuration-api.

Request body (required): `application/json` → FileConfigurationModel.

Responses: `200` OK.

## /locate/api

### POST — locate

Tags: locate-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| latitude | query | True | number (double) |  |
| longitude | query | True | number (double) |  |
| name | query | True | string |  |
| remarks | query | True | string |  |

Responses: `200` OK.

## /login/auth

### GET — handleAuthRequest

Tags: o-auth-api.

Responses: `200` OK.

## /login/authserver

### GET — getAuthServerName

Tags: o-auth-api.

Responses: `200` */* → ApiResponseString.

## /logout

### GET — logout

Tags: o-auth-api.

Responses: `200` OK.

### POST — logout_1

Tags: o-auth-api.

Responses: `200` OK.

## /register/admin/invite

### POST — invite

Tags: registration-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| emailAddress | query | True | string |  |
| group | query | False | array[string] | ["__ANON__"] |

Responses: `200` OK.

## /register/admin/users

### GET — getAllUsers_1

Tags: registration-api.

Responses: `200` */* → array[TAKUser].

## /register/token/{token}

### GET — confirm

Tags: registration-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| token | path | True | string |  |

Responses: `200` OK.

## /register/user

### POST — signUp

Tags: registration-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| emailAddress | query | True | string |  |
| token | query | True | string |  |

Responses: `200` OK.

## /token/access

### GET — getAccessToken

Tags: o-auth-api.

Responses: `200` */* → ApiResponseString.

## /user-management/api/change-user-password

### PUT — changeUserPassword

Tags: file-user-account-management-api.

Request body (required): `application/json` → UserPasswordModel.

Responses: `200` OK.

## /user-management/api/delete-user/{username}

### DELETE — deleteUser

Tags: file-user-account-management-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| username | path | True | string |  |

Responses: `200` OK.

## /user-management/api/get-groups-for-user/{username}

### GET — getGroupsForUsers

Tags: file-user-account-management-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| username | path | True | string |  |

Responses: `200` */* → SimpleUserGroupModel.

## /user-management/api/list-groupnames

### GET — getAllGroupNames

Tags: file-user-account-management-api.

Responses: `200` */* → array[GroupNameModel].

## /user-management/api/list-users

### GET — getAllUsers

Tags: file-user-account-management-api.

Responses: `200` */* → array[UsernameModel].

## /user-management/api/new-user

### POST — createOrUpdateFileUser

Tags: file-user-account-management-api.

Request body (required): `application/json` → NewUserModel.

Responses: `200` OK.

## /user-management/api/new-users

### POST — createFileUsersInBulk

Tags: file-user-account-management-api.

Request body (required): `application/json` → UserGenerationInBulkModel.

Responses: `200` */* → array[UserPasswordModel].

## /user-management/api/update-groups

### PUT — updateGroupsForUser

Tags: file-user-account-management-api.

Request body (required): `application/json` → SimpleUserGroupModel.

Responses: `200` OK.

## /user-management/api/users-in-group/{group}

### GET — getUsersInGroup

Tags: file-user-account-management-api.

| Parameter | Location | Required | Type | Default |
|---|---|---|---|---|
| group | path | True | string |  |

Responses: `200` */* → SimpleGroupWithUsersModel.

## /vbm/api/classification

### GET — getVBMNetworkClassification

Tags: vbm-configuration-api.

Responses: `200` */* → string.

## /vbm/api/config

### GET — getVBMConfiguration

Tags: vbm-configuration-api.

Responses: `200` */* → VBMConfigurationModel.

### POST — setVBMConfiguration

Tags: vbm-configuration-api.

Request body (required): `application/json` → VBMConfigurationModel.

Responses: `200` OK.

