use serde::{Deserialize, Serialize};

use crate::models::common::Activity;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct IndicatorStreamRequest {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<IndicatorType>,
    #[serde(rename = "threat_type", skip_serializing_if = "Option::is_none")]
    pub threat_type: Option<ThreatType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<Confidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_family_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub girs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndicatorType {
    Domain,
    Email,
    File,
    Ipv4,
    Url,
    Yara,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreatType {
    Malware,
    BulletproofHosting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IndicatorsStream {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub indicators: Option<Vec<IntegrationsIndicator>>,
}

impl crate::models::common::StreamPage for IndicatorsStream {
    type Item = IntegrationsIndicator;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.indicators.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IntegrationsIndicator {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: IndicatorType,
    pub activity: Option<Activity>,
    pub classification: Option<Classification>,
    pub confidence: Option<i32>,
    pub data: Option<IndicatorData>,
    pub description: Option<String>,
    pub expiration_ts: Option<String>,
    pub kill_chain_phases: Option<Vec<KillChainPhase>>,
    pub pattern: Option<String>,
    pub pattern_type: Option<String>,
    pub pattern_version: Option<String>,
    pub revocation: Option<Revocation>,
    pub threat: Option<Threat>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IndicatorData {
    pub domain: Option<String>,
    pub email: Option<String>,
    pub file: Option<File>,
    pub ipv4: Option<Ipv4>,
    pub url: Option<String>,
    pub yara: Option<YaraData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct File {
    pub md5: Option<String>,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
    pub size: Option<i64>,
    pub ssdeep: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Ipv4 {
    pub ip_address: Option<String>,
    pub geo_ip: Option<GeoIp>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeoIp {
    pub city: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub isp: Option<IspData>,
    pub subdivision: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IspData {
    pub autonomous_system: Option<String>,
    pub isp: Option<String>,
    pub network: Option<String>,
    pub organization: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct YaraData {
    pub signature: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Classification {
    pub girs: Option<Vec<Gir>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Gir {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KillChainPhase {
    pub kill_chain_name: Option<String>,
    pub phase_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Revocation {
    pub revoked: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Threat {
    #[serde(rename = "type")]
    pub type_: Option<ThreatType>,
    pub data: Option<ThreatData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ThreatData {
    pub bulletproof_hosting: Option<BulletproofHosting>,
    pub malware: Option<Malware>,
    pub malware_family: Option<MalwareFamily>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulletproofHosting {
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Malware {
    pub id: String,
    pub family: Option<String>,
    pub variant: Option<String>,
    pub version: Option<String>,
    pub component: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MalwareFamily {
    pub id: String,
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indicator_type_roundtrip() {
        let json = serde_json::to_string(&IndicatorType::Ipv4).unwrap();
        assert_eq!(json, "\"ipv4\"");
        let rt: IndicatorType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, IndicatorType::Ipv4));

        let json = serde_json::to_string(&IndicatorType::Yara).unwrap();
        assert_eq!(json, "\"yara\"");
        let rt: IndicatorType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, IndicatorType::Yara));
    }

    #[test]
    fn test_confidence_roundtrip() {
        let json = serde_json::to_string(&Confidence::High).unwrap();
        assert_eq!(json, "\"high\"");
        let rt: Confidence = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, Confidence::High));
    }

    #[test]
    fn test_kill_chain_phase_deserialize() {
        let json = r#"{"kill_chain_name":"kill-chain-name","phase_name":"reconnaissance"}"#;
        let phase: KillChainPhase = serde_json::from_str(json).unwrap();
        assert_eq!(phase.kill_chain_name, Some("kill-chain-name".to_string()));
        assert_eq!(phase.phase_name, Some("reconnaissance".to_string()));
    }

    #[test]
    fn test_indicators_stream_deserialize() {
        let json = r#"{"count":1,"cursor_next":"cursor_abc","indicators":[{"id":"ind-1","type":"domain","activity":{"first_seen_ts":"2023-01-01T00:00:00Z","last_seen_ts":"2023-06-01T00:00:00Z"},"data":{"domain":"evil.example.com"}}]}"#;
        let stream: IndicatorsStream = serde_json::from_str(json).unwrap();
        assert_eq!(stream.count, 1);
        assert_eq!(stream.cursor_next, Some("cursor_abc".to_string()));
        let indicators = stream.indicators.unwrap();
        assert_eq!(indicators[0].id, "ind-1");
        assert!(matches!(indicators[0].type_, IndicatorType::Domain));
    }

    #[test]
    fn test_indicator_stream_request_serialize() {
        let req = IndicatorStreamRequest {
            type_: Some(IndicatorType::Domain),
            threat_type: Some(ThreatType::Malware),
            confidence: Some(Confidence::Medium),
            cursor: None,
            text_filter: Some("evil.com".to_string()),
            malware_id: None,
            malware_family_id: None,
            malware_family_name: None,
            girs: None,
            size: Some(100),
            from: None,
            until: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"type\":\"domain\""));
        assert!(json.contains("\"threat_type\":\"malware\""));
        assert!(json.contains("\"confidence\":\"medium\""));
        assert!(!json.contains("cursor"));
    }
}