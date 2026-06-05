use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TprmMonitorDefinition {
    pub name: String,
    pub targets: Vec<String>,
    pub labels: Vec<String>,
    pub frequency: Frequency,
    pub impact: Impact,
    #[serde(default = "default_collection_method")]
    pub collection_method: CollectionMethod,
    pub alerts: Option<AlertOptions>,
    pub created_at: Option<u32>,
    pub start_at: Option<u32>,
}

fn default_collection_method() -> CollectionMethod {
    CollectionMethod::passive
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

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
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
    pub definition: TprmMonitorDefinition,
    pub stats: Vec<FindingStats>,
    pub last_run: Option<u32>,
    pub next_run: Option<u32>,
    pub status: Option<MonitorStatus>,
    #[serde(default)]
    pub threat: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MonitorStatus {
    CREATED,
    INITIALISING,
    STARTED,
    STARTING,
    RUNNING,
    CORRELATING,
    FINISHED,
    ABORTED,
    #[serde(rename = "ABORT-REQUESTED")]
    ABORT_REQUESTED,
    #[serde(rename = "ERROR-FAILED")]
    ERROR_FAILED,
    #[serde(rename = "POST PROCESSING")]
    POST_PROCESSING,
    OPEN,
    #[serde(rename = "ANALYSIS RUNNING")]
    ANALYSIS_RUNNING,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Scan {
    pub id: String,
    pub created_at: i64,
    pub started_at: i64,
    pub ended_at: i64,
    pub status: ScanStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ScanStatus {
    CREATED,
    INITIALISING,
    STARTED,
    STARTING,
    RUNNING,
    CORRELATING,
    FINISHED,
    ABORTED,
    #[serde(rename = "ABORT-REQUESTED")]
    ABORT_REQUESTED,
    #[serde(rename = "ERROR-FAILED")]
    ERROR_FAILED,
    #[serde(rename = "POST PROCESSING")]
    POST_PROCESSING,
    OPEN,
    #[serde(rename = "ANALYSIS RUNNING")]
    ANALYSIS_RUNNING,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScanData {
    pub id: String,
    pub data: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub observed_at: i64,
    pub links: Vec<String>,
    pub images: Vec<String>,
    pub starred: bool,
    pub note: Option<String>,
    pub occurrences: i64,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScanDataResponse {
    pub next: String,
    pub previous: String,
    pub elements: Vec<ScanData>,
    pub totals: ScanDataTotals,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScanDataTotals {
    pub total: i64,
    pub categories: std::collections::HashMap<String, i64>,
    pub types: std::collections::HashMap<String, i64>,
    pub sources: std::collections::HashMap<String, i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Node {
    pub id: String,
    pub value: String,
    pub risky: bool,
    #[serde(rename = "type")]
    pub type_: String,
    pub family: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Edge {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataGraphResponse {
    pub totals: ScanDataTotals,
    pub graph: DataGraph,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TreeNode {
    pub id: String,
    pub value: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub source: String,
    pub children: Vec<TreeNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ScanDataSort {
    #[serde(rename = "type")]
    Type,
    module,
    children,
    distance,
    instances,
    data,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Section {
    base,
    passive,
    essential,
    full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogEntry {
    pub timestamp: f64,
    #[serde(rename = "type")]
    pub type_: LogEntryType,
    pub message: String,
    pub component: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum LogEntryType {
    INFO,
    STATUS,
    ERROR,
    FATAL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MonitorLogs {
    pub monitor_id: String,
    pub next: String,
    pub previous: String,
    pub logs: Vec<LogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMonitorResponse {
    pub id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMonitorRequest {
    pub name: String,
    pub targets: Vec<String>,
    pub labels: Vec<String>,
    pub frequency: Frequency,
    pub impact: Impact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<AlertOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct EditMonitorRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<Impact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<AlertOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ListMonitorsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_after: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_before: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetMonitorLogsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<Vec<LogEntryType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backward: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetScanDataRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<ScanDataSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descending: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backward: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PatchScanDataElementRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConfigurationOutput {
    pub enabled_modules: Option<Vec<String>>,
    pub firehoses: Option<Vec<serde_json::Value>>,
    pub general: Option<GeneralConfig>,
    pub modules: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    pub rules: Option<std::collections::HashMap<String, RuleConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeneralConfig {
    #[serde(rename = "_fetchtimeout")]
    pub fetch_timeout: Option<i64>,
    #[serde(rename = "_filter_cloud")]
    pub filter_cloud: Option<bool>,
    #[serde(rename = "_genericusers")]
    pub generic_users: Option<String>,
    #[serde(rename = "_torenable")]
    pub tor_enable: Option<bool>,
    #[serde(rename = "_useragent")]
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RuleConfig {
    pub enabled: bool,
    pub risk: RiskLabel,
    pub threat: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OpenApiSchema {
    pub components: Option<Components>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Components {
    pub schemas: Schemas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Schemas {
    pub enabled_modules: serde_json::Value,
    pub modules: serde_json::Value,
    pub rules: serde_json::Value,
    pub firehoses: serde_json::Value,
    pub general: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataEvent {
    pub id: String,
    pub data: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub observed_at: i64,
    pub links: Vec<String>,
    pub images: Vec<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MonitorImpactStats {
    pub current: i64,
    pub promoted: i64,
    pub past: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MonitorRiskStats {
    pub epoch: u32,
    pub risk_histogram: std::collections::HashMap<RiskLabel, MonitorRiskHistogramBucket>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MonitorRiskHistogramBucket {
    pub risk_min: f64,
    pub risk_sup: f64,
    pub current: i64,
    pub promoted: i64,
    pub past: i64,
    pub by_impact: Option<std::collections::HashMap<Impact, MonitorImpactStats>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_status_with_rename_deserialize() {
        let json = "\"ABORT-REQUESTED\"";
        let status: MonitorStatus = serde_json::from_str(json).unwrap();
        assert!(matches!(status, MonitorStatus::ABORT_REQUESTED));

        let json = "\"ERROR-FAILED\"";
        let status: MonitorStatus = serde_json::from_str(json).unwrap();
        assert!(matches!(status, MonitorStatus::ERROR_FAILED));

        let json = "\"POST PROCESSING\"";
        let status: MonitorStatus = serde_json::from_str(json).unwrap();
        assert!(matches!(status, MonitorStatus::POST_PROCESSING));

        let json = "\"ANALYSIS RUNNING\"";
        let status: MonitorStatus = serde_json::from_str(json).unwrap();
        assert!(matches!(status, MonitorStatus::ANALYSIS_RUNNING));

        let json = "\"RUNNING\"";
        let status: MonitorStatus = serde_json::from_str(json).unwrap();
        assert!(matches!(status, MonitorStatus::RUNNING));
    }

    #[test]
    fn test_scan_status_with_rename_deserialize() {
        let json = "\"ABORT-REQUESTED\"";
        let status: ScanStatus = serde_json::from_str(json).unwrap();
        assert!(matches!(status, ScanStatus::ABORT_REQUESTED));

        let json = "\"ERROR-FAILED\"";
        let status: ScanStatus = serde_json::from_str(json).unwrap();
        assert!(matches!(status, ScanStatus::ERROR_FAILED));
    }

    #[test]
    fn test_section_roundtrip() {
        let json = serde_json::to_string(&Section::essential).unwrap();
        assert_eq!(json, "\"essential\"");
        let rt: Section = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, Section::essential));
    }

    #[test]
    fn test_log_entry_type_roundtrip() {
        let json = serde_json::to_string(&LogEntryType::ERROR).unwrap();
        assert_eq!(json, "\"ERROR\"");
        let rt: LogEntryType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, LogEntryType::ERROR));
    }

    #[test]
    fn test_risk_label_roundtrip() {
        let json = serde_json::to_string(&RiskLabel::CRITICAL).unwrap();
        assert_eq!(json, "\"CRITICAL\"");
        let rt: RiskLabel = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, RiskLabel::CRITICAL));
    }

    #[test]
    fn test_collection_method_roundtrip() {
        let json = serde_json::to_string(&CollectionMethod::passive).unwrap();
        assert_eq!(json, "\"passive\"");
        let rt: CollectionMethod = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, CollectionMethod::passive));
    }

    #[test]
    fn test_monitor_info_deserialize() {
        let json = r#"{"id":"tprm-mon-1","definition":{"name":"Test Monitor","targets":["acme.com"],"labels":["tprm"],"frequency":"daily","impact":"moderate","collection_method":"essential"},"stats":[{"finding_count":3,"risk":1.5}],"last_run":1700000000,"next_run":1700086400,"status":"RUNNING","threat":true}"#;
        let info: MonitorInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.id, "tprm-mon-1");
        assert_eq!(info.definition.name, "Test Monitor");
        assert!(info.threat);
    }

    #[test]
    fn test_scan_data_deserialize() {
        let json = r#"{"id":"sd-1","data":"192.168.1.1","type":"ipv4","observed_at":1700000000,"links":[],"images":[],"starred":false,"occurrences":5,"sources":["passive"]}"#;
        let data: ScanData = serde_json::from_str(json).unwrap();
        assert_eq!(data.id, "sd-1");
        assert_eq!(data.type_, "ipv4");
        assert_eq!(data.occurrences, 5);
    }

    #[test]
    fn test_log_entry_deserialize() {
        let json = r#"{"timestamp":1700000000.123,"type":"STATUS","message":"Scan started","component":"scanner"}"#;
        let entry: LogEntry = serde_json::from_str(json).unwrap();
        assert!(matches!(entry.type_, LogEntryType::STATUS));
        assert_eq!(entry.message, "Scan started");
    }

    #[test]
    fn test_configuration_output_deserialize() {
        let json = r#"{"enabled_modules":["module1","module2"],"firehoses":[],"general":{"_fetchtimeout":30,"_filter_cloud":true,"_genericusers":"generic","_torenable":false,"_useragent":"Mozilla/5.0"},"modules":{},"rules":{"rule1":{"enabled":true,"risk":"HIGH","threat":false}}}"#;
        let config: ConfigurationOutput = serde_json::from_str(json).unwrap();
        assert_eq!(config.enabled_modules.unwrap().len(), 2);
        assert!(config.rules.unwrap().contains_key("rule1"));
    }
}