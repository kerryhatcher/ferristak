//! Cursor-on-Target (CoT) events.
//!
//! This module models the **CoT base event** as defined by the public MITRE
//! schema (v2.0, updated for MIL-STD-6090) and the `detail` sub-schemas in
//! `xsd/details/*.xsd`. The Rust types map 1:1 onto the XML:
//!
//! ```xml
//! <event version="2.0" type="a-f-G-U-C" access="Unclassified" uid="…"
//!        time="…" start="…" stale="…" how="m-g">
//!   <point lat="…" lon="…" hae="…" ce="…" le="…"/>
//!   <detail>
//!     <contact callsign="…"/>
//!     <__group name="…" role="…"/>
//!     <status battery="…"/>
//!     <track speed="…" course="…"/>
//!     <takv device="…" platform="…" os="…" version="…"/>
//!     <precisionlocation geopointsrc="…" altsrc="…"/>
//!     <remarks>…</remarks>
//!   </detail>
//! </event>
//! ```
//!
//! Value semantics follow the schema: `Point`, `Contact`, `Group`, `Status`,
//! `Track`, `Takv` and `PrecisionLocation` are value objects; `CotEvent` is an
//! entity (identified by `id` and the CoT `uid`). Validation mirrors the XSD
//! restrictions (ranges, required attributes and `xs:pattern`s).

mod xml;

use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::error::AppError;

pub use xml::{from_xml, to_xml, to_xml_list};

/// Sentinel used by CoT when a point metric is unknown.
pub const UNKNOWN_METRIC: f64 = 999_999.0;

// The CoT grammar is ASCII (`xs:pattern` with `\w` is `[A-Za-z0-9_]` in XML
// Schema). Rust's `\w` is Unicode-aware, so use explicit classes instead.

/// CoT base-event `type` grammar: `word(-word)*(;optional)`.
static TYPE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Za-z0-9_]+(-[A-Za-z0-9_]+)*(;[^;]*)?$").expect("valid type regex")
});

/// CoT `how` grammar: `word(-word)*`.
static HOW_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z0-9_](-[A-Za-z0-9_]+)*$").expect("valid how regex"));

/// CoT `qos` grammar: `digit-word-word`.
static QOS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[0-9]-[A-Za-z0-9_]-[A-Za-z0-9_]$").expect("valid qos regex"));

/// CoT `opex` grammar: `o|e|s` with an optional `-name`.
static OPEX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[oes](-.+)?$").expect("valid opex regex"));

// ---------------------------------------------------------------------------
// Value objects
// ---------------------------------------------------------------------------

/// A CoT `<point>`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
    #[serde(default = "unknown_metric")]
    pub hae: f64,
    #[serde(default = "unknown_metric")]
    pub ce: f64,
    #[serde(default = "unknown_metric")]
    pub le: f64,
}

impl Point {
    fn validate(&self) -> Result<(), AppError> {
        ensure(
            self.lat.is_finite() && (-90.0..=90.0).contains(&self.lat),
            "point.lat must be between -90 and 90",
        )?;
        ensure(
            self.lon.is_finite() && (-180.0..=180.0).contains(&self.lon),
            "point.lon must be between -180 and 180",
        )?;
        ensure(self.hae.is_finite(), "point.hae must be finite")?;
        ensure(self.ce.is_finite(), "point.ce must be finite")?;
        ensure(self.le.is_finite(), "point.le must be finite")?;
        Ok(())
    }
}

/// A CoT `<contact>` detail element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub callsign: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

/// A CoT `<__group>` detail element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub role: String,
}

/// A CoT `<status>` detail element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub battery: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readiness: Option<bool>,
}

/// A CoT `<track>` detail element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub speed: f64,
    pub course: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slope: Option<f64>,
}

/// A CoT `<takv>` detail element (device/version information).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Takv {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    pub platform: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    pub version: String,
}

/// A CoT `<precisionlocation>` detail element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrecisionLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geopointsrc: Option<String>,
    pub altsrc: String,
}

/// The `<detail>` element: the strongly-typed subset we persist, in schema order.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Detail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(rename = "__group", default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track: Option<Track>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub takv: Option<Takv>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precisionlocation: Option<PrecisionLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}

impl Detail {
    /// True when no strongly-typed detail is present.
    pub fn is_empty(&self) -> bool {
        self.contact.is_none()
            && self.group.is_none()
            && self.status.is_none()
            && self.track.is_none()
            && self.takv.is_none()
            && self.precisionlocation.is_none()
            && self.remarks.is_none()
    }

    fn validate(&self) -> Result<(), AppError> {
        if let Some(contact) = &self.contact {
            ensure(
                !contact.callsign.trim().is_empty(),
                "detail.contact.callsign must not be empty",
            )?;
        }
        if let Some(group) = &self.group {
            ensure(
                !group.name.trim().is_empty(),
                "detail.__group.name must not be empty",
            )?;
            ensure(
                !group.role.trim().is_empty(),
                "detail.__group.role must not be empty",
            )?;
        }
        if let Some(status) = &self.status {
            if let Some(battery) = status.battery {
                ensure(
                    (0..=100).contains(&battery),
                    "detail.status.battery must be between 0 and 100",
                )?;
            }
        }
        if let Some(track) = &self.track {
            ensure(track.speed.is_finite(), "detail.track.speed must be finite")?;
            ensure(
                track.course.is_finite() && (0.0..=360.0).contains(&track.course),
                "detail.track.course must be between 0 and 360",
            )?;
            if let Some(slope) = track.slope {
                ensure(slope.is_finite(), "detail.track.slope must be finite")?;
            }
        }
        if let Some(takv) = &self.takv {
            ensure(
                !takv.platform.trim().is_empty(),
                "detail.takv.platform must not be empty",
            )?;
            ensure(
                !takv.version.trim().is_empty(),
                "detail.takv.version must not be empty",
            )?;
        }
        if let Some(precision) = &self.precisionlocation {
            ensure(
                !precision.altsrc.trim().is_empty(),
                "detail.precisionlocation.altsrc must not be empty",
            )?;
        }
        if let Some(remarks) = &self.remarks {
            ensure(
                remarks.len() <= 4096,
                "detail.remarks must be at most 4096 characters",
            )?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Event
// ---------------------------------------------------------------------------

/// An inbound CoT event (request body). `uid` is optional so an operator can
/// check in without inventing one; it defaults to `ferristak-<owner id>`.
#[derive(Debug, Clone, Deserialize)]
pub struct NewCotEvent {
    #[serde(default = "default_version")]
    pub version: f64,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(default = "default_access")]
    pub access: String,
    #[serde(default)]
    pub caveat: Option<String>,
    #[serde(default, alias = "releasableTo")]
    pub releasable_to: Option<String>,
    #[serde(default)]
    pub qos: Option<String>,
    #[serde(default)]
    pub opex: Option<String>,
    #[serde(default)]
    pub uid: Option<String>,
    #[serde(with = "cot_time")]
    pub time: DateTime<Utc>,
    #[serde(with = "cot_time")]
    pub start: DateTime<Utc>,
    #[serde(with = "cot_time")]
    pub stale: DateTime<Utc>,
    pub how: String,
    pub point: Point,
    #[serde(default)]
    pub detail: Detail,
    /// The original CoT `<event>` element exactly as received, when this event
    /// arrived as XML. It is the authoritative representation for XML output;
    /// the typed fields above are only an index over it. Empty for JSON input,
    /// where XML output is synthesised from the typed fields instead.
    #[serde(default)]
    pub raw_xml: String,
}

impl NewCotEvent {
    /// Validate against the CoT schema restrictions.
    pub fn validate(&self) -> Result<(), AppError> {
        ensure(self.version >= 2.0, "version must be >= 2.0")?;
        ensure(
            TYPE_RE.is_match(&self.event_type),
            "type must be a valid CoT type (e.g. a-f-G-U-C)",
        )?;
        ensure(!self.access.trim().is_empty(), "access must not be empty")?;
        if let Some(qos) = &self.qos {
            ensure(
                QOS_RE.is_match(qos),
                "qos must be <priority>-<overtaking>-<assurance> (e.g. 1-r-c)",
            )?;
        }
        if let Some(opex) = &self.opex {
            ensure(
                OPEX_RE.is_match(opex),
                "opex must be o[-name], e[-nickname] or s[-nickname]",
            )?;
        }
        ensure(
            HOW_RE.is_match(&self.how),
            "how must be a valid CoT how code (e.g. m-g)",
        )?;
        if let Some(uid) = &self.uid {
            ensure(
                !uid.trim().is_empty(),
                "uid must not be empty when provided",
            )?;
        }
        ensure(
            self.start <= self.stale,
            "start must be before or equal to stale",
        )?;
        ensure(
            self.start <= self.time && self.time <= self.stale,
            "time must be between start and stale",
        )?;
        self.point.validate()?;
        self.detail.validate()?;
        Ok(())
    }

    /// The CoT uid to persist, defaulting to one derived from the owner.
    pub fn uid_or_default(&self, owner_id: Uuid) -> String {
        self.uid
            .as_deref()
            .map(str::trim)
            .filter(|uid| !uid.is_empty())
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| format!("ferristak-{owner_id}"))
    }
}

/// A persisted CoT event, in its nested (schema-shaped) form.
#[derive(Debug, Clone, Serialize)]
pub struct CotEvent {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub version: f64,
    #[serde(rename = "type")]
    pub event_type: String,
    pub access: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caveat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub releasable_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opex: Option<String>,
    pub uid: String,
    #[serde(with = "cot_time")]
    pub time: DateTime<Utc>,
    #[serde(with = "cot_time")]
    pub start: DateTime<Utc>,
    #[serde(with = "cot_time")]
    pub stale: DateTime<Utc>,
    pub how: String,
    pub point: Point,
    pub detail: Detail,
    /// Original `<event>` element for XML submissions; see `NewCotEvent::raw_xml`.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub raw_xml: String,
    #[serde(with = "cot_time")]
    pub created_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Persistence
// ---------------------------------------------------------------------------

const COLUMNS: &str = "id, owner_id, cot_version, event_type, access, caveat, releasable_to, qos, \
     opex, uid, send_time, start_time, stale_time, how, lat, lon, hae, ce, le, \
     contact_callsign, contact_endpoint, group_name, group_role, battery, readiness, \
     track_speed, track_course, track_slope, takv_device, takv_platform, takv_os, \
     takv_version, geopointsrc, altsrc, remarks, raw_xml, created_at";

/// Flat database representation of a CoT event.
#[derive(Debug, Clone, FromRow)]
struct CotEventRow {
    id: Uuid,
    owner_id: Uuid,
    cot_version: f64,
    event_type: String,
    access: String,
    caveat: Option<String>,
    releasable_to: Option<String>,
    qos: Option<String>,
    opex: Option<String>,
    uid: String,
    send_time: DateTime<Utc>,
    start_time: DateTime<Utc>,
    stale_time: DateTime<Utc>,
    how: String,
    lat: f64,
    lon: f64,
    hae: f64,
    ce: f64,
    le: f64,
    contact_callsign: Option<String>,
    contact_endpoint: Option<String>,
    group_name: Option<String>,
    group_role: Option<String>,
    battery: Option<i32>,
    readiness: Option<bool>,
    track_speed: Option<f64>,
    track_course: Option<f64>,
    track_slope: Option<f64>,
    takv_device: Option<String>,
    takv_platform: Option<String>,
    takv_os: Option<String>,
    takv_version: Option<String>,
    geopointsrc: Option<String>,
    altsrc: Option<String>,
    remarks: Option<String>,
    raw_xml: String,
    created_at: DateTime<Utc>,
}

impl From<CotEventRow> for CotEvent {
    fn from(row: CotEventRow) -> Self {
        let contact = row.contact_callsign.map(|callsign| Contact {
            callsign,
            endpoint: row.contact_endpoint,
        });

        let group = match (row.group_name, row.group_role) {
            (Some(name), Some(role)) => Some(Group { name, role }),
            _ => None,
        };

        let status = match (row.battery, row.readiness) {
            (None, None) => None,
            (battery, readiness) => Some(Status { battery, readiness }),
        };

        let track = row.track_speed.map(|speed| Track {
            speed,
            course: row.track_course.unwrap_or_default(),
            slope: row.track_slope,
        });

        let takv = row.takv_platform.map(|platform| Takv {
            device: row.takv_device,
            platform,
            os: row.takv_os,
            version: row.takv_version.unwrap_or_default(),
        });

        let precisionlocation = row.altsrc.map(|altsrc| PrecisionLocation {
            geopointsrc: row.geopointsrc,
            altsrc,
        });

        Self {
            id: row.id,
            owner_id: row.owner_id,
            version: row.cot_version,
            event_type: row.event_type,
            access: row.access,
            caveat: row.caveat,
            releasable_to: row.releasable_to,
            qos: row.qos,
            opex: row.opex,
            uid: row.uid,
            time: row.send_time,
            start: row.start_time,
            stale: row.stale_time,
            how: row.how,
            point: Point {
                lat: row.lat,
                lon: row.lon,
                hae: row.hae,
                ce: row.ce,
                le: row.le,
            },
            detail: Detail {
                contact,
                group,
                status,
                track,
                takv,
                precisionlocation,
                remarks: row.remarks,
            },
            raw_xml: row.raw_xml,
            created_at: row.created_at,
        }
    }
}

/// Insert a CoT event.
pub async fn insert(
    pool: &PgPool,
    owner_id: Uuid,
    uid: &str,
    input: &NewCotEvent,
) -> Result<CotEvent, AppError> {
    let detail = &input.detail;
    let contact = detail.contact.as_ref();
    let group = detail.group.as_ref();
    let status = detail.status.as_ref();
    let track = detail.track.as_ref();
    let takv = detail.takv.as_ref();
    let precision = detail.precisionlocation.as_ref();

    let sql = "insert into cot_events ( \
            owner_id, cot_version, event_type, access, caveat, releasable_to, qos, opex, \
            uid, send_time, start_time, stale_time, how, lat, lon, hae, ce, le, \
            contact_callsign, contact_endpoint, group_name, group_role, battery, readiness, \
            track_speed, track_course, track_slope, takv_device, takv_platform, takv_os, \
            takv_version, geopointsrc, altsrc, remarks, raw_xml \
         ) values ( \
            $1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20, \
            $21,$22,$23,$24,$25,$26,$27,$28,$29,$30,$31,$32,$33,$34,$35 \
         ) returning id, owner_id, cot_version, event_type, access, caveat, releasable_to, qos, \
            opex, uid, send_time, start_time, stale_time, how, lat, lon, hae, ce, le, \
            contact_callsign, contact_endpoint, group_name, group_role, battery, readiness, \
            track_speed, track_course, track_slope, takv_device, takv_platform, takv_os, \
            takv_version, geopointsrc, altsrc, remarks, raw_xml, created_at";

    let row = sqlx::query_as::<_, CotEventRow>(sql)
        .bind(owner_id)
        .bind(input.version)
        .bind(&input.event_type)
        .bind(&input.access)
        .bind(&input.caveat)
        .bind(&input.releasable_to)
        .bind(&input.qos)
        .bind(&input.opex)
        .bind(uid)
        .bind(input.time)
        .bind(input.start)
        .bind(input.stale)
        .bind(&input.how)
        .bind(input.point.lat)
        .bind(input.point.lon)
        .bind(input.point.hae)
        .bind(input.point.ce)
        .bind(input.point.le)
        .bind(contact.map(|c| c.callsign.as_str()))
        .bind(contact.and_then(|c| c.endpoint.as_deref()))
        .bind(group.map(|g| g.name.as_str()))
        .bind(group.map(|g| g.role.as_str()))
        .bind(status.and_then(|s| s.battery))
        .bind(status.and_then(|s| s.readiness))
        .bind(track.map(|t| t.speed))
        .bind(track.map(|t| t.course))
        .bind(track.and_then(|t| t.slope))
        .bind(takv.and_then(|t| t.device.as_deref()))
        .bind(takv.map(|t| t.platform.as_str()))
        .bind(takv.and_then(|t| t.os.as_deref()))
        .bind(takv.map(|t| t.version.as_str()))
        .bind(precision.and_then(|p| p.geopointsrc.as_deref()))
        .bind(precision.map(|p| p.altsrc.as_str()))
        .bind(detail.remarks.as_deref())
        .bind(input.raw_xml.as_str())
        .fetch_one(pool)
        .await?;

    Ok(row.into())
}

/// List events, newest first, optionally filtered by time and uid.
pub async fn list(
    pool: &PgPool,
    limit: i64,
    since: Option<DateTime<Utc>>,
    uid: Option<&str>,
    owner: Option<Uuid>,
) -> Result<Vec<CotEvent>, AppError> {
    let sql = format!(
        "select {COLUMNS} from cot_events \
         where ($1::timestamptz is null or send_time >= $1) \
           and ($2::text is null or uid = $2) \
           and ($3::uuid is null or owner_id = $3) \
         order by send_time desc limit $4"
    );

    let rows = sqlx::query_as::<_, CotEventRow>(sqlx::AssertSqlSafe(sql))
        .bind(since)
        .bind(uid)
        .bind(owner)
        .bind(limit)
        .fetch_all(pool)
        .await?;

    Ok(rows.into_iter().map(Into::into).collect())
}

/// Fetch a single event by id.
pub async fn find(
    pool: &PgPool,
    id: Uuid,
    owner: Option<Uuid>,
) -> Result<Option<CotEvent>, AppError> {
    let sql = format!(
        "select {COLUMNS} from cot_events \
         where id = $1 and ($2::uuid is null or owner_id = $2)"
    );

    let row = sqlx::query_as::<_, CotEventRow>(sqlx::AssertSqlSafe(sql))
        .bind(id)
        .bind(owner)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(Into::into))
}

/// Current situational picture: the newest event for each uid.
pub async fn latest(
    pool: &PgPool,
    limit: i64,
    owner: Option<Uuid>,
) -> Result<Vec<CotEvent>, AppError> {
    let sql = format!(
        "select * from ( \
            select distinct on (uid) {COLUMNS} from cot_events \
            where ($1::uuid is null or owner_id = $1) \
            order by uid, send_time desc \
         ) latest order by send_time desc limit $2"
    );

    let rows = sqlx::query_as::<_, CotEventRow>(sqlx::AssertSqlSafe(sql))
        .bind(owner)
        .bind(limit)
        .fetch_all(pool)
        .await?;

    Ok(rows.into_iter().map(Into::into).collect())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// CoT timestamps are RFC 3339 with a `Z` suffix and optional fractional seconds.
pub(crate) mod cot_time {
    use chrono::{DateTime, SecondsFormat, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_rfc3339_opts(SecondsFormat::Millis, true))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&raw)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
    }
}

pub(crate) fn unknown_metric() -> f64 {
    UNKNOWN_METRIC
}

fn default_version() -> f64 {
    2.0
}

fn default_access() -> String {
    "Undefined".to_string()
}

pub(crate) fn ensure(condition: bool, message: &'static str) -> Result<(), AppError> {
    if condition {
        Ok(())
    } else {
        Err(AppError::validation(message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn sample() -> NewCotEvent {
        let t = Utc.with_ymd_and_hms(2026, 10, 21, 15, 49, 23).unwrap();
        NewCotEvent {
            version: 2.0,
            event_type: "a-f-G-U-C".to_string(),
            access: "Unclassified".to_string(),
            caveat: None,
            releasable_to: None,
            qos: Some("1-r-c".to_string()),
            opex: None,
            uid: Some("ANDROID-1234".to_string()),
            time: t,
            start: t,
            stale: t + chrono::Duration::minutes(5),
            how: "m-g".to_string(),
            point: Point {
                lat: 34.05,
                lon: -118.25,
                hae: UNKNOWN_METRIC,
                ce: 5.0,
                le: 5.0,
            },
            detail: Detail {
                contact: Some(Contact {
                    callsign: "Ranger-1".to_string(),
                    endpoint: Some("*:-1:stcp".to_string()),
                }),
                group: Some(Group {
                    name: "Cyan".to_string(),
                    role: "Team Member".to_string(),
                }),
                status: Some(Status {
                    battery: Some(87),
                    readiness: None,
                }),
                ..Detail::default()
            },
            raw_xml: String::new(),
        }
    }

    #[test]
    fn valid_event_passes() {
        sample().validate().unwrap();
    }

    #[test]
    fn rejects_non_ascii_type() {
        let mut event = sample();
        event.event_type = "a-f-G-Ü".to_string();
        assert!(event.validate().is_err());
    }

    #[test]
    fn rejects_time_outside_validity_window() {
        let mut event = sample();
        event.time = event.stale + chrono::Duration::seconds(1);
        assert!(event.validate().is_err());

        let mut event = sample();
        event.time = event.start - chrono::Duration::seconds(1);
        assert!(event.validate().is_err());
    }

    #[test]
    fn rejects_bad_type_and_how() {
        let mut event = sample();
        event.event_type = "not a cot type".to_string();
        assert!(event.validate().is_err());

        let mut event = sample();
        event.how = "-bad".to_string();
        assert!(event.validate().is_err());
    }

    #[test]
    fn rejects_out_of_range_point() {
        let mut event = sample();
        event.point.lat = 120.0;
        assert!(event.validate().is_err());

        let mut event = sample();
        event.point.lon = -400.0;
        assert!(event.validate().is_err());
    }

    #[test]
    fn rejects_inverted_validity_interval() {
        let mut event = sample();
        event.stale = event.start - chrono::Duration::seconds(1);
        assert!(event.validate().is_err());
    }

    #[test]
    fn rejects_bad_battery() {
        let mut event = sample();
        event.detail.status = Some(Status {
            battery: Some(140),
            readiness: None,
        });
        assert!(event.validate().is_err());
    }

    #[test]
    fn uid_defaults_to_owner() {
        let mut event = sample();
        event.uid = None;
        let owner = Uuid::nil();
        assert_eq!(event.uid_or_default(owner), format!("ferristak-{owner}"));
    }
}
