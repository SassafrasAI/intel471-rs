use serde::{Deserialize, Serialize};

use crate::models::common::{Activity, Links, StreamPage};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportType {
    Fintel,
    InfoReport,
    BreachAlert,
    SpotReport,
    VulnerabilityReport,
    MalwareReport,
    GeopolReport,
    SituationReport,
    NewsReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FintelReportSubType {
    ActorProfile,
    IntelligenceBulletin,
    ServiceProfile,
    UndergroundPerspective,
    UndergroundPulse,
    Whitepaper,
    ThreatBrief,
    BreachReport,
    IntelligenceSummary,
    MalwareCampaign,
    FintelBlog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeopolReportSubType {
    SpotReport,
    IntelligenceBulletin,
    IntelligenceSummary,
    TensionPointProfile,
    ThreatBrief,
    SignificantActivityReport,
    IntelligenceEstimate,
    AdversaryProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityLocation {
    LocationOpensource,
    LocationUnderground,
    LocationPrivate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExploitStatus {
    Available,
    Weaponized,
    Productized,
    NotObserved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatchStatus {
    Available,
    SomeAvailable,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterestLevel {
    DisclosedPublicly,
    ExploitSought,
    ResearchedPublicly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Poc {
    Observed,
    NotObserved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VulnerabilityStatus {
    New,
    Existing,
    Historical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdmiraltyCode {
    #[serde(rename = "A1")]
    A1,
    #[serde(rename = "A2")]
    A2,
    #[serde(rename = "A3")]
    A3,
    #[serde(rename = "A4")]
    A4,
    #[serde(rename = "A5")]
    A5,
    #[serde(rename = "A6")]
    A6,
    #[serde(rename = "B1")]
    B1,
    #[serde(rename = "B2")]
    B2,
    #[serde(rename = "B3")]
    B3,
    #[serde(rename = "B4")]
    B4,
    #[serde(rename = "B5")]
    B5,
    #[serde(rename = "B6")]
    B6,
    #[serde(rename = "C1")]
    C1,
    #[serde(rename = "C2")]
    C2,
    #[serde(rename = "C3")]
    C3,
    #[serde(rename = "C4")]
    C4,
    #[serde(rename = "C5")]
    C5,
    #[serde(rename = "C6")]
    C6,
    #[serde(rename = "D1")]
    D1,
    #[serde(rename = "D2")]
    D2,
    #[serde(rename = "D3")]
    D3,
    #[serde(rename = "D4")]
    D4,
    #[serde(rename = "D5")]
    D5,
    #[serde(rename = "D6")]
    D6,
    #[serde(rename = "E1")]
    E1,
    #[serde(rename = "E2")]
    E2,
    #[serde(rename = "E3")]
    E3,
    #[serde(rename = "E4")]
    E4,
    #[serde(rename = "E5")]
    E5,
    #[serde(rename = "E6")]
    E6,
    #[serde(rename = "F1")]
    F1,
    #[serde(rename = "F2")]
    F2,
    #[serde(rename = "F3")]
    F3,
    #[serde(rename = "F4")]
    F4,
    #[serde(rename = "F5")]
    F5,
    #[serde(rename = "F6")]
    F6,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventTag {
    Economic,
    Social,
    Military,
    Political,
    Legal,
    Infotech,
    Cyber,
    Environmental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Motivation {
    #[serde(rename = "HA")]
    HA,
    #[serde(rename = "CC")]
    CC,
    #[serde(rename = "CE")]
    CE,
    #[serde(rename = "DE")]
    DE,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityAssessment {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreatRating {
    Low,
    Moderate,
    Substantial,
    Severe,
    Critical,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReportsStreamRequest {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<Vec<ReportType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub girs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReportResponseStream {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub reports: Option<Vec<ReportContent>>,
    pub fintel_report_count: Option<i64>,
    pub info_report_count: Option<i64>,
    pub breach_alert_count: Option<i64>,
    pub spot_report_count: Option<i64>,
    pub malware_report_count: Option<i64>,
    pub vulnerability_report_count: Option<i64>,
    pub geopol_report_count: Option<i64>,
}

impl StreamPage for ReportResponseStream {
    type Item = ReportContent;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.reports.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize)]
pub struct FintelStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<Vec<FintelReportSubType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub girs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FintelReportsResponseStream {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub reports: Option<Vec<FintelResponse>>,
}

impl StreamPage for FintelReportsResponseStream {
    type Item = FintelResponse;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.reports.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize)]
pub struct BreachAlertStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub girs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BreachAlertsResponseStream {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub reports: Option<Vec<BreachAlertResponse>>,
}

impl StreamPage for BreachAlertsResponseStream {
    type Item = BreachAlertResponse;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.reports.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize)]
pub struct GeopolStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<Vec<GeopolReportSubType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_location_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub girs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeopolReportsResponseStream {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub reports: Option<Vec<GeopolReportDetailsResponse>>,
    pub spot_count: Option<i64>,
    pub intelligence_bulletin_count: Option<i64>,
    pub intelligence_summary_count: Option<i64>,
    pub tension_point_profile_count: Option<i64>,
    pub adversary_profile_count: Option<i64>,
    pub threat_brief_count: Option<i64>,
    pub sigact_count: Option<i64>,
    pub intelligence_estimate_count: Option<i64>,
}

impl StreamPage for GeopolReportsResponseStream {
    type Item = GeopolReportDetailsResponse;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.reports.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize)]
pub struct InfoStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub girs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InfoReportsResponseStream {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub reports: Option<Vec<InfoReportResponse>>,
}

impl StreamPage for InfoReportsResponseStream {
    type Item = InfoReportResponse;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.reports.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize)]
pub struct MalwareStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub girs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MalwareReportsResponseStream {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub reports: Option<Vec<MalwareReportResponse>>,
}

impl StreamPage for MalwareReportsResponseStream {
    type Item = MalwareReportResponse;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.reports.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize)]
pub struct SpotStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub girs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SpotReportsResponseStream {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub reports: Option<Vec<SpotReportResponse>>,
}

impl StreamPage for SpotReportsResponseStream {
    type Item = SpotReportResponse;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.reports.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize)]
pub struct VulnerabilityStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VulnerabilityStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<Vec<RiskLevel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_status: Option<Vec<PatchStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_level: Option<Vec<InterestLevel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_location: Option<Vec<ActivityLocation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploit_status: Option<Vec<ExploitStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub girs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VulnerabilitiesReportsResponseStream {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub reports: Option<Vec<VulnerabilitiesReportDetailsResponseStream>>,
}

impl StreamPage for VulnerabilitiesReportsResponseStream {
    type Item = VulnerabilitiesReportDetailsResponseStream;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.reports.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Classification {
    pub girs: Option<Vec<Gir>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Gir {
    pub name: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Confidence {
    pub level: Option<ConfidenceLevel>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Entities {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Assessment {
    pub admiralty_code: Option<AdmiraltyCode>,
    pub confidence: Option<Confidence>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReportAttachment {
    pub url: Option<String>,
    pub file_name: Option<String>,
    pub malicious: Option<bool>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReportContent {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<ReportType>,
    pub sub_type: Option<crate::models::common::ReportSubType>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub creation_ts: Option<String>,
    pub released_ts: Option<String>,
    pub last_updated_ts: Option<String>,
    pub information_ts: Option<String>,
    pub classification: Option<Classification>,
    pub assessment: Option<Assessment>,
    pub links: Option<Links>,
    pub sources: Option<Vec<SourcesResponse>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SourcesResponse {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub title: Option<String>,
    pub source_type: Option<String>,
    pub last_updated_ts: Option<String>,
    pub index: Option<i32>,
    pub links: Option<Links>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReportLocation {
    pub region: Option<String>,
    pub country: Option<String>,
    pub iso: Option<String>,
    pub link: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ActorSubjectOfReport {
    pub handle: Option<String>,
    pub aliases: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Industries {
    pub industry: Option<String>,
    pub sector: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReportsVictimResponse {
    pub name: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub revenue: Option<String>,
    pub industries: Option<Vec<Industries>>,
    pub links: Option<Vec<Links>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LinksSource {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub links: Option<Links>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ThreatInfo {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub family: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CountryProfileResponse {
    pub id: Option<String>,
    pub country: Option<String>,
    pub country_iso_code: Option<String>,
    pub information_ts: Option<String>,
    pub is_country_of_interest: Option<bool>,
    pub security_assessment: Option<SecurityAssessment>,
    pub threat_rating: Option<ThreatRating>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IntelligenceEstimateResponse {
    pub is_country_of_interest: Option<bool>,
    pub overview: Option<String>,
    pub report_location: Option<ReportLocation>,
    pub security_assessment: Option<SecurityAssessment>,
    pub text_toc: Option<String>,
    pub threat_rating: Option<ThreatRating>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignificantActivity {
    pub comments: Option<String>,
    pub event_tag: Option<EventTag>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TensionPointResponse {
    pub id: Option<String>,
    pub name: Option<String>,
    pub information_ts: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Cvss {
    pub version: Option<String>,
    pub score: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FintelResponse {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub sub_type: Option<FintelReportSubType>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub creation_ts: Option<String>,
    pub last_updated_ts: Option<String>,
    pub information_ts: Option<String>,
    pub released_ts: Option<String>,
    pub is_sensitive_source: Option<bool>,
    pub is_truncated: Option<bool>,
    pub classification: Option<Classification>,
    pub links: Option<Links>,
    pub entities: Option<Vec<Entities>>,
    pub derived_entities: Option<Vec<Entities>>,
    pub locations: Option<Vec<ReportLocation>>,
    pub sources: Option<Vec<SourcesResponse>>,
    pub related_reports: Option<Vec<ReportContent>>,
    pub victims: Option<Vec<ReportsVictimResponse>>,
    pub attachments: Option<Vec<ReportAttachment>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BreachAlertResponse {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub creation_ts: Option<String>,
    pub last_updated_ts: Option<String>,
    pub information_ts: Option<String>,
    pub released_ts: Option<String>,
    pub is_sensitive_source: Option<bool>,
    pub is_truncated: Option<bool>,
    pub classification: Option<Classification>,
    pub actor_or_group: Option<String>,
    pub confidence: Option<Confidence>,
    pub links: Option<Links>,
    pub entities: Option<Vec<Entities>>,
    pub sources: Option<Vec<SourcesResponse>>,
    pub related_reports: Option<Vec<ReportContent>>,
    pub victims: Option<Vec<ReportsVictimResponse>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeopolReportDetailsResponse {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub sub_type: Option<GeopolReportSubType>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub creation_ts: Option<String>,
    pub last_updated_ts: Option<String>,
    pub information_ts: Option<String>,
    pub released_ts: Option<String>,
    pub is_sensitive_source: Option<bool>,
    pub is_truncated: Option<bool>,
    pub classification: Option<Classification>,
    pub links: Option<Links>,
    pub entities: Option<Vec<Entities>>,
    pub derived_entities: Option<Vec<Entities>>,
    pub locations: Option<Vec<ReportLocation>>,
    pub sources: Option<Vec<SourcesResponse>>,
    pub related_reports: Option<Vec<ReportContent>>,
    pub country_profiles: Option<Vec<CountryProfileResponse>>,
    pub regional_tension_points: Option<Vec<TensionPointResponse>>,
    pub significant_activity: Option<SignificantActivity>,
    pub intelligence_estimate: Option<IntelligenceEstimateResponse>,
    pub attachments: Option<Vec<ReportAttachment>>,
    pub active_event: Option<bool>,
    pub victims: Option<Vec<ReportsVictimResponse>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InfoReportResponse {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub body_translated: Option<String>,
    pub creation_ts: Option<String>,
    pub last_updated_ts: Option<String>,
    pub information_ts: Option<String>,
    pub released_ts: Option<String>,
    pub is_sensitive_source: Option<bool>,
    pub is_truncated: Option<bool>,
    pub classification: Option<Classification>,
    pub assessment: Option<Assessment>,
    pub links: Option<Links>,
    pub entities: Option<Vec<Entities>>,
    pub derived_entities: Option<Vec<Entities>>,
    pub locations: Option<Vec<ReportLocation>>,
    pub actor_subject_of_report: Option<Vec<ActorSubjectOfReport>>,
    pub motivation: Option<Vec<Motivation>>,
    pub sources: Option<Vec<SourcesResponse>>,
    pub related_reports: Option<Vec<ReportContent>>,
    pub executive_summary: Option<String>,
    pub researcher_comments: Option<String>,
    pub source_characterization: Option<String>,
    pub victims: Option<Vec<ReportsVictimResponse>>,
    pub attachments: Option<Vec<ReportAttachment>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MalwareReportResponse {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub creation_ts: Option<String>,
    pub last_updated_ts: Option<String>,
    pub information_ts: Option<String>,
    pub released_ts: Option<String>,
    pub is_sensitive_source: Option<bool>,
    pub is_truncated: Option<bool>,
    pub version: Option<String>,
    pub classification: Option<Classification>,
    pub threat: Option<ThreatInfo>,
    pub links: Option<Links>,
    pub entities: Option<Vec<Entities>>,
    pub derived_entities: Option<Vec<Entities>>,
    pub sources: Option<Vec<SourcesResponse>>,
    pub related_reports: Option<Vec<ReportContent>>,
    pub attachments: Option<Vec<ReportAttachment>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SpotReportResponse {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub creation_ts: Option<String>,
    pub last_updated_ts: Option<String>,
    pub information_ts: Option<String>,
    pub released_ts: Option<String>,
    pub is_sensitive_source: Option<bool>,
    pub is_truncated: Option<bool>,
    pub classification: Option<Classification>,
    pub links: Option<Links>,
    pub entities: Option<Vec<Entities>>,
    pub derived_entities: Option<Vec<Entities>>,
    pub sources: Option<Vec<SourcesResponse>>,
    pub related_reports: Option<Vec<ReportContent>>,
    pub victims: Option<Vec<ReportsVictimResponse>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VulnerabilitiesReportDetailsResponse {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub name: Option<String>,
    pub body: Option<String>,
    pub creation_ts: Option<String>,
    pub last_updated_ts: Option<String>,
    pub information_ts: Option<String>,
    pub released_ts: Option<String>,
    pub status: Option<VulnerabilityStatus>,
    pub cve_type: Option<String>,
    pub risk_level: Option<RiskLevel>,
    pub vendor_name: Option<String>,
    pub product_name: Option<String>,
    pub classification: Option<Classification>,
    pub activity: Option<Activity>,
    pub activity_location: Option<Vec<ActivityLocation>>,
    pub aliases: Option<Vec<String>>,
    pub counter_measure_links: Option<Vec<LinksSource>>,
    pub counter_measures_html: Option<String>,
    pub cvss: Option<Vec<Cvss>>,
    pub exploit_status: Option<Vec<ExploitStatus>>,
    pub interest_level: Option<Vec<InterestLevel>>,
    pub links: Option<Links>,
    pub patch_links: Option<Vec<LinksSource>>,
    pub patch_status: Option<PatchStatus>,
    pub poc: Option<Poc>,
    pub poc_links: Option<Vec<LinksSource>>,
    pub sort_priority: Option<i32>,
    pub sources: Option<Vec<SourcesResponse>>,
    pub underground_activity_summary_html: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_type_roundtrip() {
        let json = serde_json::to_string(&ReportType::Fintel).unwrap();
        assert_eq!(json, "\"fintel\"");
        let rt: ReportType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, ReportType::Fintel));

        let json = serde_json::to_string(&ReportType::VulnerabilityReport).unwrap();
        assert_eq!(json, "\"vulnerability_report\"");
        let rt: ReportType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, ReportType::VulnerabilityReport));
    }

    #[test]
    fn test_admiralty_code_roundtrip() {
        let json = serde_json::to_string(&AdmiraltyCode::A1).unwrap();
        assert_eq!(json, "\"A1\"");
        let rt: AdmiraltyCode = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, AdmiraltyCode::A1));

        let json = serde_json::to_string(&AdmiraltyCode::F6).unwrap();
        assert_eq!(json, "\"F6\"");
        let rt: AdmiraltyCode = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, AdmiraltyCode::F6));

        let json = serde_json::to_string(&AdmiraltyCode::C3).unwrap();
        assert_eq!(json, "\"C3\"");
        let rt: AdmiraltyCode = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, AdmiraltyCode::C3));
    }

    #[test]
    fn test_vulnerability_status_roundtrip() {
        let json = serde_json::to_string(&VulnerabilityStatus::New).unwrap();
        assert_eq!(json, "\"new\"");
        let rt: VulnerabilityStatus = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, VulnerabilityStatus::New));
    }

    #[test]
    fn test_motivation_roundtrip() {
        let json = serde_json::to_string(&Motivation::HA).unwrap();
        assert_eq!(json, "\"HA\"");
        let rt: Motivation = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, Motivation::HA));

        let json = serde_json::to_string(&Motivation::CC).unwrap();
        assert_eq!(json, "\"CC\"");
        let rt: Motivation = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, Motivation::CC));
    }

    #[test]
    fn test_report_response_stream_deserialize() {
        let json = r#"{"count":1,"cursor_next":"cursor_abc","reports":[{"id":"rpt-1","type":"fintel","title":"Test Report","creation_ts":"2023-01-01T00:00:00Z","last_updated_ts":"2023-06-01T00:00:00Z"}],"fintel_report_count":1,"info_report_count":0,"breach_alert_count":0,"spot_report_count":0,"malware_report_count":0,"vulnerability_report_count":0,"geopol_report_count":0}"#;
        let stream: ReportResponseStream = serde_json::from_str(json).unwrap();
        assert_eq!(stream.count, 1);
        assert_eq!(stream.fintel_report_count, Some(1));
        let reports = stream.reports.unwrap();
        assert_eq!(reports[0].id, Some("rpt-1".to_string()));
    }

    #[test]
    fn test_confidence_level_roundtrip() {
        let json = serde_json::to_string(&ConfidenceLevel::High).unwrap();
        assert_eq!(json, "\"high\"");
        let rt: ConfidenceLevel = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, ConfidenceLevel::High));
    }

    #[test]
    fn test_event_tag_roundtrip() {
        let json = serde_json::to_string(&EventTag::Cyber).unwrap();
        assert_eq!(json, "\"cyber\"");
        let rt: EventTag = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, EventTag::Cyber));
    }

    #[test]
    fn test_threat_rating_roundtrip() {
        let json = serde_json::to_string(&ThreatRating::Critical).unwrap();
        assert_eq!(json, "\"critical\"");
        let rt: ThreatRating = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, ThreatRating::Critical));
    }
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VulnerabilitiesReportDetailsResponseStream {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub name: Option<String>,
    pub body: Option<String>,
    pub creation_ts: Option<String>,
    pub last_updated_ts: Option<String>,
    pub released_ts: Option<String>,
    pub status: Option<VulnerabilityStatus>,
    pub cve_type: Option<String>,
    pub risk_level: Option<RiskLevel>,
    pub vendor_name: Option<String>,
    pub product_name: Option<String>,
    pub classification: Option<Classification>,
    pub activity: Option<Activity>,
    pub activity_location: Option<Vec<ActivityLocation>>,
    pub aliases: Option<Vec<String>>,
    pub counter_measure_links: Option<Vec<LinksSource>>,
    pub counter_measures_html: Option<String>,
    pub cvss: Option<Vec<Cvss>>,
    pub exploit_status: Option<Vec<ExploitStatus>>,
    pub interest_level: Option<Vec<InterestLevel>>,
    pub links: Option<Links>,
    pub patch_links: Option<Vec<LinksSource>>,
    pub patch_status: Option<PatchStatus>,
    pub poc: Option<Poc>,
    pub poc_links: Option<Vec<LinksSource>>,
    pub sort_priority: Option<i32>,
    pub sources: Option<Vec<SourcesResponse>>,
    pub underground_activity_summary_html: Option<String>,
}