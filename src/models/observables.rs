use serde::{Deserialize, Serialize};

use crate::models::common::{Activity, Report};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ObservableStreamRequest {
    pub observable: String,
    #[serde(rename = "type")]
    pub type_: Option<ObservableType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObservableType {
    #[serde(rename = "AutonomousSystem")]
    AutonomousSystem,
    #[serde(rename = "FileSize")]
    FileSize,
    #[serde(rename = "FileType")]
    FileType,
    #[serde(rename = "FileName")]
    FileName,
    #[serde(rename = "IPAddress")]
    IPAddress,
    #[serde(rename = "MaliciousDomain")]
    MaliciousDomain,
    #[serde(rename = "MaliciousURL")]
    MaliciousURL,
    #[serde(rename = "MD5")]
    MD5,
    #[serde(rename = "SHA1")]
    SHA1,
    #[serde(rename = "SHA256")]
    SHA256,
    #[serde(rename = "SSLCertificate")]
    SSLCertificate,
    #[serde(rename = "SSLCertificateFingerprint")]
    SSLCertificateFingerprint,
    #[serde(rename = "SSLCertificateID")]
    SSLCertificateID,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ObservableStreamPage {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub observables: Option<Vec<Observable>>,
}

impl crate::models::common::StreamPage for ObservableStreamPage {
    type Item = Observable;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.observables.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Observable {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ObservableType,
    pub value: String,
    pub geo_ip: Option<GeoIp>,
    pub activity: Option<Activity>,
    pub report: Option<Report>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeoIp {
    pub asn: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub isp: Option<IspInfo>,
    pub organization: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IspInfo {
    pub network: Option<String>,
    pub autonomous_system: Option<String>,
    pub isp: Option<String>,
    pub organization: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observable_type_explicit_rename_roundtrip() {
        let json = serde_json::to_string(&ObservableType::IPAddress).unwrap();
        assert_eq!(json, "\"IPAddress\"");
        let rt: ObservableType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, ObservableType::IPAddress));

        let json = serde_json::to_string(&ObservableType::MD5).unwrap();
        assert_eq!(json, "\"MD5\"");
        let rt: ObservableType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, ObservableType::MD5));

        let json = serde_json::to_string(&ObservableType::SHA256).unwrap();
        assert_eq!(json, "\"SHA256\"");
        let rt: ObservableType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, ObservableType::SHA256));

        let json = serde_json::to_string(&ObservableType::MaliciousDomain).unwrap();
        assert_eq!(json, "\"MaliciousDomain\"");
        let rt: ObservableType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, ObservableType::MaliciousDomain));

        let json = serde_json::to_string(&ObservableType::SSLCertificate).unwrap();
        assert_eq!(json, "\"SSLCertificate\"");
        let rt: ObservableType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, ObservableType::SSLCertificate));
    }

    #[test]
    fn test_observable_stream_page_deserialize() {
        let json = r#"{"count":2,"cursor_next":"next_cursor","observables":[{"id":"obs-1","type":"IPAddress","value":"192.168.1.1","activity":{"first_seen_ts":"2023-01-01","last_seen_ts":"2023-06-01"}},{"id":"obs-2","type":"MD5","value":"d41d8cd98f00b204e9800998ecf8427e"}]}"#;
        let page: ObservableStreamPage = serde_json::from_str(json).unwrap();
        assert_eq!(page.count, 2);
        assert_eq!(page.cursor_next, Some("next_cursor".to_string()));
        let observables = page.observables.unwrap();
        assert_eq!(observables.len(), 2);
        assert!(matches!(observables[0].type_, ObservableType::IPAddress));
        assert!(matches!(observables[1].type_, ObservableType::MD5));
    }

    #[test]
    fn test_observable_stream_request_serialize() {
        let req = ObservableStreamRequest {
            observable: "192.168.1.1".to_string(),
            type_: Some(ObservableType::IPAddress),
            from: Some(1627776000000),
            until: None,
            size: Some(50),
            cursor: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"observable\":\"192.168.1.1\""));
        assert!(json.contains("\"type\":\"IPAddress\""));
        assert!(!json.contains("until"));
    }
}