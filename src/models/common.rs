use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Activity {
    pub first_seen_ts: String,
    pub last_seen_ts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Report {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ReportType,
    pub sub_type: Option<ReportSubType>,
    pub creation_ts: String,
    pub last_updated_ts: String,
    pub information_ts: String,
    pub released_ts: String,
    pub links: Links,
}

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
pub enum ReportSubType {
    FintelBlog,
    AdversaryProfile,
    ThreatBrief,
    IntelligenceSummary,
    MalwareCampaign,
    SpotReport,
    UndergroundPulse,
    BreachReport,
    ActorProfile,
    ServiceProfile,
    Whitepaper,
    UndergroundPerspective,
    TensionPointProfile,
    IntelligenceBulletin,
    IntelligenceEstimate,
    SignificantActivityReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Links {
    pub verity_api: Option<Link>,
    pub verity_portal: Option<Link>,
    pub external: Option<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Link {
    pub href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatServerType {
    #[serde(rename = "Telegram")]
    Telegram,
    #[serde(rename = "Discord")]
    Discord,
    #[serde(rename = "IRC")]
    IRC,
    #[serde(rename = "WhatsApp")]
    WhatsApp,
    #[serde(rename = "QQ")]
    QQ,
    #[serde(rename = "ICQ")]
    ICQ,
    #[serde(rename = "Matrix")]
    Matrix,
    #[serde(rename = "Signal")]
    Signal,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApiErrorResponse {
    pub error: String,
    pub timestamp: Option<String>,
}

pub trait StreamPage {
    type Item;
    fn count(&self) -> i64;
    fn cursor_next(&self) -> Option<String>;
    fn items(&self) -> &[Self::Item];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_type_roundtrip() {
        let json = serde_json::to_string(&ReportType::BreachAlert).unwrap();
        assert_eq!(json, "\"breach_alert\"");
        let rt: ReportType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, ReportType::BreachAlert));
    }

    #[test]
    fn test_report_sub_type_roundtrip() {
        let json = serde_json::to_string(&ReportSubType::FintelBlog).unwrap();
        assert_eq!(json, "\"fintel_blog\"");
        let rt: ReportSubType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, ReportSubType::FintelBlog));
    }

    #[test]
    fn test_activity_deserialize() {
        let json = r#"{"first_seen_ts":"2023-01-01T00:00:00Z","last_seen_ts":"2023-06-01T00:00:00Z"}"#;
        let activity: Activity = serde_json::from_str(json).unwrap();
        assert_eq!(activity.first_seen_ts, "2023-01-01T00:00:00Z");
        assert_eq!(activity.last_seen_ts, "2023-06-01T00:00:00Z");
    }

    #[test]
    fn test_links_deserialize() {
        let json = r#"{"verity_api":{"href":"https://api.example.com"},"verity_portal":{"href":"https://portal.example.com"}}"#;
        let links: Links = serde_json::from_str(json).unwrap();
        assert!(links.external.is_none());
        assert_eq!(links.verity_api.unwrap().href, "https://api.example.com");
    }

    #[test]
    fn test_chat_server_type_roundtrip() {
        let json = serde_json::to_string(&ChatServerType::Telegram).unwrap();
        assert_eq!(json, "\"Telegram\"");
        let rt: ChatServerType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, ChatServerType::Telegram));
    }
}