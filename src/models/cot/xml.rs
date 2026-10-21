//! The CoT XML wire format, expressed with `quick-xml`'s serde support
//! (`@attr` for attributes, `$text` for element text).
//!
//! The wire types are kept separate from the JSON-facing domain types so that
//! attributes never leak into JSON keys (and vice-versa).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    cot_time, unknown_metric, Contact, CotEvent, Detail, Group, NewCotEvent, Point,
    PrecisionLocation, Status, Takv, Track,
};
use crate::error::AppError;

/// `<event …>` — the CoT root element.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "event")]
pub struct CotEventXml {
    #[serde(rename = "@version", with = "cot_version")]
    pub version: f64,
    #[serde(rename = "@type")]
    pub event_type: String,
    #[serde(rename = "@access")]
    pub access: String,
    #[serde(rename = "@caveat", default, skip_serializing_if = "Option::is_none")]
    pub caveat: Option<String>,
    #[serde(
        rename = "@releasableto",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub releasable_to: Option<String>,
    #[serde(rename = "@qos", default, skip_serializing_if = "Option::is_none")]
    pub qos: Option<String>,
    #[serde(rename = "@opex", default, skip_serializing_if = "Option::is_none")]
    pub opex: Option<String>,
    #[serde(rename = "@uid")]
    pub uid: String,
    #[serde(rename = "@time", with = "cot_time")]
    pub time: DateTime<Utc>,
    #[serde(rename = "@start", with = "cot_time")]
    pub start: DateTime<Utc>,
    #[serde(rename = "@stale", with = "cot_time")]
    pub stale: DateTime<Utc>,
    #[serde(rename = "@how")]
    pub how: String,
    #[serde(rename = "point")]
    pub point: PointXml,
    #[serde(rename = "detail", default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<DetailXml>,
}

/// `<point …>`
#[derive(Debug, Serialize, Deserialize)]
pub struct PointXml {
    #[serde(rename = "@lat")]
    pub lat: f64,
    #[serde(rename = "@lon")]
    pub lon: f64,
    #[serde(rename = "@hae", default = "unknown_metric")]
    pub hae: f64,
    #[serde(rename = "@ce", default = "unknown_metric")]
    pub ce: f64,
    #[serde(rename = "@le", default = "unknown_metric")]
    pub le: f64,
}

/// `<contact …>`
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactXml {
    #[serde(rename = "@callsign")]
    pub callsign: String,
    #[serde(rename = "@endpoint", default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

/// `<__group …>`
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupXml {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@role")]
    pub role: String,
}

/// `<status …>`
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusXml {
    #[serde(rename = "@battery", default, skip_serializing_if = "Option::is_none")]
    pub battery: Option<i32>,
    #[serde(
        rename = "@readiness",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub readiness: Option<bool>,
}

/// `<track …>`
#[derive(Debug, Serialize, Deserialize)]
pub struct TrackXml {
    #[serde(rename = "@speed")]
    pub speed: f64,
    #[serde(rename = "@course")]
    pub course: f64,
    #[serde(rename = "@slope", default, skip_serializing_if = "Option::is_none")]
    pub slope: Option<f64>,
}

/// `<takv …>`
#[derive(Debug, Serialize, Deserialize)]
pub struct TakvXml {
    #[serde(rename = "@device", default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(rename = "@platform")]
    pub platform: String,
    #[serde(rename = "@os", default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "@version")]
    pub version: String,
}

/// `<precisionlocation …>`
#[derive(Debug, Serialize, Deserialize)]
pub struct PrecisionLocationXml {
    #[serde(
        rename = "@geopointsrc",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub geopointsrc: Option<String>,
    #[serde(rename = "@altsrc")]
    pub altsrc: String,
}

/// `<remarks>…</remarks>`
#[derive(Debug, Serialize, Deserialize)]
pub struct RemarksXml {
    #[serde(rename = "$text")]
    pub text: String,
}

/// `<detail>` — children emitted in schema order.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DetailXml {
    #[serde(rename = "contact", default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactXml>,
    #[serde(rename = "__group", default, skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupXml>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusXml>,
    #[serde(rename = "track", default, skip_serializing_if = "Option::is_none")]
    pub track: Option<TrackXml>,
    #[serde(rename = "takv", default, skip_serializing_if = "Option::is_none")]
    pub takv: Option<TakvXml>,
    #[serde(
        rename = "precisionlocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub precisionlocation: Option<PrecisionLocationXml>,
    #[serde(rename = "remarks", default, skip_serializing_if = "Option::is_none")]
    pub remarks: Option<RemarksXml>,
}

// ---------------------------------------------------------------------------
// Domain <-> wire conversions
// ---------------------------------------------------------------------------

impl From<&Point> for PointXml {
    fn from(p: &Point) -> Self {
        Self {
            lat: p.lat,
            lon: p.lon,
            hae: p.hae,
            ce: p.ce,
            le: p.le,
        }
    }
}

impl From<PointXml> for Point {
    fn from(p: PointXml) -> Self {
        Self {
            lat: p.lat,
            lon: p.lon,
            hae: p.hae,
            ce: p.ce,
            le: p.le,
        }
    }
}

impl From<&Contact> for ContactXml {
    fn from(c: &Contact) -> Self {
        Self {
            callsign: c.callsign.clone(),
            endpoint: c.endpoint.clone(),
        }
    }
}

impl From<ContactXml> for Contact {
    fn from(c: ContactXml) -> Self {
        Self {
            callsign: c.callsign,
            endpoint: c.endpoint,
        }
    }
}

impl From<&Group> for GroupXml {
    fn from(g: &Group) -> Self {
        Self {
            name: g.name.clone(),
            role: g.role.clone(),
        }
    }
}

impl From<GroupXml> for Group {
    fn from(g: GroupXml) -> Self {
        Self {
            name: g.name,
            role: g.role,
        }
    }
}

impl From<&Status> for StatusXml {
    fn from(s: &Status) -> Self {
        Self {
            battery: s.battery,
            readiness: s.readiness,
        }
    }
}

impl From<StatusXml> for Status {
    fn from(s: StatusXml) -> Self {
        Self {
            battery: s.battery,
            readiness: s.readiness,
        }
    }
}

impl From<&Track> for TrackXml {
    fn from(t: &Track) -> Self {
        Self {
            speed: t.speed,
            course: t.course,
            slope: t.slope,
        }
    }
}

impl From<TrackXml> for Track {
    fn from(t: TrackXml) -> Self {
        Self {
            speed: t.speed,
            course: t.course,
            slope: t.slope,
        }
    }
}

impl From<&Takv> for TakvXml {
    fn from(t: &Takv) -> Self {
        Self {
            device: t.device.clone(),
            platform: t.platform.clone(),
            os: t.os.clone(),
            version: t.version.clone(),
        }
    }
}

impl From<TakvXml> for Takv {
    fn from(t: TakvXml) -> Self {
        Self {
            device: t.device,
            platform: t.platform,
            os: t.os,
            version: t.version,
        }
    }
}

impl From<&PrecisionLocation> for PrecisionLocationXml {
    fn from(p: &PrecisionLocation) -> Self {
        Self {
            geopointsrc: p.geopointsrc.clone(),
            altsrc: p.altsrc.clone(),
        }
    }
}

impl From<PrecisionLocationXml> for PrecisionLocation {
    fn from(p: PrecisionLocationXml) -> Self {
        Self {
            geopointsrc: p.geopointsrc,
            altsrc: p.altsrc,
        }
    }
}

fn detail_to_xml(detail: &Detail) -> Option<DetailXml> {
    if detail.is_empty() {
        return None;
    }

    Some(DetailXml {
        contact: detail.contact.as_ref().map(ContactXml::from),
        group: detail.group.as_ref().map(GroupXml::from),
        status: detail.status.as_ref().map(StatusXml::from),
        track: detail.track.as_ref().map(TrackXml::from),
        takv: detail.takv.as_ref().map(TakvXml::from),
        precisionlocation: detail
            .precisionlocation
            .as_ref()
            .map(PrecisionLocationXml::from),
        remarks: detail
            .remarks
            .as_ref()
            .map(|text| RemarksXml { text: text.clone() }),
    })
}

impl From<DetailXml> for Detail {
    fn from(d: DetailXml) -> Self {
        Self {
            contact: d.contact.map(Contact::from),
            group: d.group.map(Group::from),
            status: d.status.map(Status::from),
            track: d.track.map(Track::from),
            takv: d.takv.map(Takv::from),
            precisionlocation: d.precisionlocation.map(PrecisionLocation::from),
            remarks: d.remarks.map(|r| r.text),
        }
    }
}

impl From<&CotEvent> for CotEventXml {
    fn from(e: &CotEvent) -> Self {
        Self {
            version: e.version,
            event_type: e.event_type.clone(),
            access: e.access.clone(),
            caveat: e.caveat.clone(),
            releasable_to: e.releasable_to.clone(),
            qos: e.qos.clone(),
            opex: e.opex.clone(),
            uid: e.uid.clone(),
            time: e.time,
            start: e.start,
            stale: e.stale,
            how: e.how.clone(),
            point: PointXml::from(&e.point),
            detail: detail_to_xml(&e.detail),
        }
    }
}

impl From<CotEventXml> for NewCotEvent {
    fn from(x: CotEventXml) -> Self {
        Self {
            version: x.version,
            event_type: x.event_type,
            access: x.access,
            caveat: x.caveat,
            releasable_to: x.releasable_to,
            qos: x.qos,
            opex: x.opex,
            uid: Some(x.uid),
            time: x.time,
            start: x.start,
            stale: x.stale,
            how: x.how,
            point: x.point.into(),
            detail: x.detail.map(Detail::from).unwrap_or_default(),
            extra_detail: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
}

// ---------------------------------------------------------------------------
// Public XML API
// ---------------------------------------------------------------------------

/// Render a single event as a CoT XML document.
pub fn to_xml(event: &CotEvent) -> Result<String, AppError> {
    let wire = CotEventXml::from(event);
    let body = quick_xml::se::to_string(&wire).map_err(se_error)?;
    Ok(format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{body}"
    ))
}

/// Render a list of events as an `<events>` document (one `<event>` per child).
pub fn to_xml_list(events: &[CotEvent]) -> Result<String, AppError> {
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<events>");
    for event in events {
        out.push_str(&quick_xml::se::to_string(&CotEventXml::from(event)).map_err(se_error)?);
    }
    out.push_str("</events>");
    Ok(out)
}

/// Parse a single CoT XML document into an inbound event.
pub fn from_xml(input: &str) -> Result<NewCotEvent, AppError> {
    let wire: CotEventXml = quick_xml::de::from_str(input).map_err(de_error)?;
    Ok(NewCotEvent::from(wire))
}

/// CoT conventionally spells the version as `2.0`. `f64` alone would render as
/// `2`, so force a single fractional digit.
mod cot_version {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{value:.1}"))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        raw.parse::<f64>().map_err(serde::de::Error::custom)
    }
}

fn de_error(err: quick_xml::DeError) -> AppError {
    AppError::validation(format!("invalid CoT XML: {err}"))
}

fn se_error(err: quick_xml::SeError) -> AppError {
    AppError::Internal(anyhow::anyhow!("failed to serialise CoT XML: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cot::UNKNOWN_METRIC;
    use chrono::TimeZone;
    use uuid::Uuid;

    fn persisted_event() -> CotEvent {
        let t = Utc.with_ymd_and_hms(2026, 10, 21, 15, 49, 23).unwrap();
        CotEvent {
            id: Uuid::nil(),
            owner_id: Uuid::nil(),
            version: 2.0,
            event_type: "a-f-G-U-C".to_string(),
            access: "Unclassified".to_string(),
            caveat: None,
            releasable_to: None,
            qos: Some("1-r-c".to_string()),
            opex: None,
            uid: "ANDROID-1234".to_string(),
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
                track: Some(Track {
                    speed: 0.0,
                    course: 270.0,
                    slope: None,
                }),
                takv: Some(Takv {
                    device: Some("pixel".to_string()),
                    platform: "ATAK".to_string(),
                    os: Some("android".to_string()),
                    version: "5.4.0".to_string(),
                }),
                precisionlocation: Some(PrecisionLocation {
                    geopointsrc: Some("GPS".to_string()),
                    altsrc: "GPS".to_string(),
                }),
                remarks: Some("on station".to_string()),
            },
            extra_detail: serde_json::Value::Object(serde_json::Map::new()),
            created_at: t,
        }
    }

    #[test]
    fn round_trips_through_xml() {
        let event = persisted_event();
        let xml = to_xml(&event).unwrap();

        // Canonical attributes and element names are present.
        assert!(xml.contains("<event "), "{xml}");
        assert!(xml.contains("version=\"2.0\""), "{xml}");
        assert!(xml.contains("type=\"a-f-G-U-C\""), "{xml}");
        assert!(xml.contains("how=\"m-g\""), "{xml}");
        assert!(
            xml.contains("<point lat=\"34.05\" lon=\"-118.25\""),
            "{xml}"
        );
        assert!(xml.contains("<contact callsign=\"Ranger-1\""), "{xml}");
        assert!(
            xml.contains("<__group name=\"Cyan\" role=\"Team Member\""),
            "{xml}"
        );
        assert!(xml.contains("<status battery=\"87\""), "{xml}");
        assert!(xml.contains("<remarks>on station</remarks>"), "{xml}");

        let parsed = from_xml(&xml).unwrap();
        assert_eq!(parsed.event_type, event.event_type);
        assert_eq!(parsed.uid.as_deref(), Some("ANDROID-1234"));
        assert_eq!(parsed.how, "m-g");
        assert_eq!(parsed.point, event.point);
        assert_eq!(parsed.detail, event.detail);
        assert_eq!(parsed.time, event.time);
    }

    #[test]
    fn parses_a_minimal_document() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
            <event version="2.0" type="a-f-G-U-C" access="Unclassified" uid="X"
                   time="2026-10-21T15:49:23Z" start="2026-10-21T15:49:23Z"
                   stale="2026-10-21T15:54:23Z" how="m-g">
              <point lat="1.5" lon="2.5" hae="999999" ce="999999" le="999999"/>
            </event>"#;

        let parsed = from_xml(xml).unwrap();
        assert_eq!(parsed.event_type, "a-f-G-U-C");
        assert_eq!(parsed.point.lat, 1.5);
        assert!(parsed.detail.is_empty());
        parsed.validate().unwrap();
    }

    #[test]
    fn list_is_wrapped() {
        let xml = to_xml_list(&[persisted_event()]).unwrap();
        assert!(xml.starts_with("<?xml"));
        assert!(xml.contains("<events>"));
        assert!(xml.contains("<event "));
    }
}
