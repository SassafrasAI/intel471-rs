use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AseMonitorDefinition {
    pub name: String,
    pub targets: Vec<String>,
    pub labels: Vec<String>,
    pub frequency: Frequency,
    pub impact: Impact,
    pub collection_method: CollectionMethod,
    pub alerts: Option<AlertOptions>,
    pub created_at: Option<u32>,
    pub disabled_modules: Option<Vec<String>>,
    pub event_types: Option<Vec<String>>,
    pub iteration: Option<IterationOptions>,
    pub start_at: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertOptions {
    pub callback_url: Option<String>,
    pub emails: Option<Vec<String>>,
    pub excluded_event_types: Option<Vec<String>>,
    pub excluded_finding_risks: Option<Vec<RiskLabel>>,
    pub slack_webhook: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum CollectionMethod {
    passive,
    essential,
    full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Frequency {
    manual,
    daily,
    monthly,
    weekly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Impact {
    insignificant,
    minor,
    moderate,
    major,
    catastrophic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationOptions {
    pub affiliates: Option<bool>,
    pub cohosts: Option<bool>,
    pub crypto: Option<bool>,
    pub emails: Option<bool>,
    pub hosts: Option<bool>,
    pub names: Option<bool>,
    pub phones: Option<bool>,
    pub users: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum RiskLabel {
    INFO,
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MonitorInfo {
    pub id: String,
    pub definition: AseMonitorDefinition,
    pub stats: Vec<FindingStats>,
    pub last_run: Option<u32>,
    pub next_run: Option<u32>,
    pub status: Option<String>,
    pub threat: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindingStats {
    pub finding_count: i64,
    pub risk: f64,
    pub created_at: Option<u32>,
    pub threat_count: Option<i64>,
    pub transaction_type: Option<String>,
    pub trigerred_by: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ListMonitorsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_after: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_before: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_method_roundtrip() {
        let json = serde_json::to_string(&CollectionMethod::essential).unwrap();
        assert_eq!(json, "\"essential\"");
        let rt: CollectionMethod = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, CollectionMethod::essential));
    }

    #[test]
    fn test_frequency_roundtrip() {
        let json = serde_json::to_string(&Frequency::daily).unwrap();
        assert_eq!(json, "\"daily\"");
        let rt: Frequency = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, Frequency::daily));
    }

    #[test]
    fn test_impact_roundtrip() {
        let json = serde_json::to_string(&Impact::major).unwrap();
        assert_eq!(json, "\"major\"");
        let rt: Impact = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, Impact::major));
    }

    #[test]
    fn test_risk_label_roundtrip() {
        let json = serde_json::to_string(&RiskLabel::CRITICAL).unwrap();
        assert_eq!(json, "\"CRITICAL\"");
        let rt: RiskLabel = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, RiskLabel::CRITICAL));
    }

    #[test]
    fn test_ase_monitor_definition_roundtrip() {
        let def = AseMonitorDefinition {
            name: "test-monitor".to_string(),
            targets: vec!["example.com".to_string()],
            labels: vec!["label1".to_string()],
            frequency: Frequency::weekly,
            impact: Impact::moderate,
            collection_method: CollectionMethod::full,
            alerts: None,
            created_at: None,
            disabled_modules: None,
            event_types: None,
            iteration: None,
            start_at: None,
        };
        let json = serde_json::to_string(&def).unwrap();
        assert!(json.contains("\"frequency\":\"weekly\""));
        assert!(json.contains("\"impact\":\"moderate\""));
        assert!(json.contains("\"collection_method\":\"full\""));
        let rt: AseMonitorDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(rt.name, "test-monitor");
        assert!(matches!(rt.frequency, Frequency::weekly));
    }

    #[test]
    fn test_monitor_info_deserialize() {
        let json = r#"{"id":"mon-123","definition":{"name":"test","targets":["intel.com"],"labels":[],"frequency":"daily","impact":"minor","collection_method":"passive"},"stats":[{"finding_count":5,"risk":2.5}],"last_run":1700000000,"next_run":1700086400,"status":"FINISHED","threat":true}"#;
        let info: MonitorInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.id, "mon-123");
        assert_eq!(info.stats.len(), 1);
        assert_eq!(info.stats[0].finding_count, 5);
        assert!(info.threat.unwrap());
    }
}