use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandExposureMonitorDefinition {
    pub name: String,
    pub targets: Vec<String>,
    pub labels: Vec<String>,
    pub frequency: Frequency,
    pub impact: Impact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<AlertOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_modules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iteration: Option<IterationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_event_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_finding_risks: Option<Vec<RiskLabel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohosts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub definition: BrandExposureMonitorDefinition,
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
    AbortRequested,
    #[serde(rename = "ERROR-FAILED")]
    ErrorFailed,
    #[serde(rename = "POST PROCESSING")]
    PostProcessing,
    OPEN,
    #[serde(rename = "ANALYSIS RUNNING")]
    AnalysisRunning,
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMonitorRequest {
    pub name: String,
    pub targets: Vec<String>,
    pub labels: Vec<String>,
    pub frequency: Frequency,
    pub impact: Impact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<AlertOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_modules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iteration: Option<IterationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct EditMonitorRequest {
    pub name: String,
    pub targets: Vec<String>,
    pub labels: Vec<String>,
    pub frequency: Frequency,
    pub impact: Impact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<AlertOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_modules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iteration: Option<IterationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMonitorResponse {
    pub id: String,
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
pub struct ConfigurationInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_modules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehoses: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<GeneralConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<HashMap<String, HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<HashMap<String, RuleConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConfigurationOutput {
    #[serde(default)]
    pub enabled_modules: Vec<String>,
    #[serde(default)]
    pub firehoses: Vec<serde_json::Value>,
    pub general: Option<GeneralConfig>,
    #[serde(default)]
    pub modules: HashMap<String, HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub rules: HashMap<String, RuleConfig>,
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
    AbortRequested,
    #[serde(rename = "ERROR-FAILED")]
    ErrorFailed,
    #[serde(rename = "POST PROCESSING")]
    PostProcessing,
    OPEN,
    #[serde(rename = "ANALYSIS RUNNING")]
    AnalysisRunning,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetMonitorScansRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<ScanStatus>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetMonitorLogsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<Vec<LogType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backward: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogType {
    INFO,
    STATUS,
    ERROR,
    FATAL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MonitorLogs {
    pub monitor_id: String,
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub previous: String,
    pub logs: Vec<LogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogEntry {
    pub timestamp: f64,
    #[serde(rename = "type")]
    pub type_: LogType,
    pub message: String,
    pub component: String,
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetScanDataGraphRequest {
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
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetScanDataTreeRequest {
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
pub struct ScanDataResponse {
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub previous: String,
    pub elements: Vec<DataElement>,
    pub totals: ScanDataTotals,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataElement {
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
pub struct ScanDataTotals {
    pub total: i64,
    pub categories: HashMap<String, i64>,
    pub types: HashMap<String, i64>,
    pub sources: HashMap<String, i64>,
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
#[serde(rename_all = "snake_case")]
pub struct ScanApiTreeNode {
    pub id: String,
    pub element_id: String,
    pub value: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub source: String,
    pub children: Vec<ScanApiTreeNode>,
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
pub struct OpenApiSchema {
    pub openapi: String,
    pub info: serde_json::Value,
    pub components: Components,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    pub schemas: Schemas,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_status_with_rename_roundtrip() {
        let json = serde_json::to_string(&MonitorStatus::AbortRequested).unwrap();
        assert_eq!(json, "\"ABORT-REQUESTED\"");
        let rt: MonitorStatus = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, MonitorStatus::AbortRequested));

        let json = serde_json::to_string(&MonitorStatus::ErrorFailed).unwrap();
        assert_eq!(json, "\"ERROR-FAILED\"");
        let rt: MonitorStatus = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, MonitorStatus::ErrorFailed));
    }

    #[test]
    fn test_create_monitor_response_deserialize() {
        let json = r#"{"id":"be-mon-456"}"#;
        let resp: CreateMonitorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.id, "be-mon-456");
    }

    #[test]
    fn test_scan_data_sort_roundtrip() {
        let json = serde_json::to_string(&ScanDataSort::Type).unwrap();
        assert_eq!(json, "\"type\"");
        let rt: ScanDataSort = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, ScanDataSort::Type));
    }

    #[test]
    fn test_log_type_roundtrip() {
        let json = serde_json::to_string(&LogType::ERROR).unwrap();
        assert_eq!(json, "\"ERROR\"");
        let rt: LogType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, LogType::ERROR));
    }

    #[test]
    fn test_monitor_info_deserialize() {
        let json = r#"{"id":"be-1","definition":{"name":"Brand Monitor","targets":["acme.com"],"labels":["brand"],"frequency":"daily","impact":"moderate"},"stats":[{"finding_count":10,"risk":3.2}],"last_run":1700000000,"next_run":1700086400,"status":"FINISHED","threat":false}"#;
        let info: MonitorInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.id, "be-1");
        assert_eq!(info.definition.name, "Brand Monitor");
        assert!(!info.threat);
    }

    #[test]
    fn test_scan_data_response_deserialize() {
        let json = r#"{"next":"cursor123","previous":"","elements":[{"id":"el-1","data":"192.168.1.1","type":"ipv4","observed_at":1700000000,"links":[],"images":[],"starred":false,"occurrences":1,"sources":["passive"]}],"totals":{"total":1,"categories":{"ipv4":1},"types":{"ipv4":1},"sources":{"passive":1}}}"#;
        let resp: ScanDataResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.elements.len(), 1);
        assert_eq!(resp.elements[0].data, "192.168.1.1");
        assert_eq!(resp.totals.total, 1);
    }

    #[test]
    fn test_configuration_output_deserialize() {
        let json = r#"{"enabled_modules":["module1"],"firehoses":[],"general":{"_fetchtimeout":30,"_filter_cloud":true},"modules":{},"rules":{}}"#;
        let config: ConfigurationOutput = serde_json::from_str(json).unwrap();
        assert_eq!(config.enabled_modules.len(), 1);
        assert!(config.general.is_some());
        let general = config.general.unwrap();
        assert_eq!(general.fetch_timeout, Some(30));
    }

    #[test]
    fn test_section_roundtrip() {
        let json = serde_json::to_string(&Section::passive).unwrap();
        assert_eq!(json, "\"passive\"");
        let rt: Section = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, Section::passive));
    }
}