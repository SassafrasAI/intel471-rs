use serde::{Deserialize, Serialize};

use crate::models::common::StreamPage;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CredentialStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_set_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliation_group: Option<AffiliationGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_strength: Option<PasswordStrength>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_length_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_lowercase_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_uppercase_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_numbers_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_punctuation_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_symbols_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_separators_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_other_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_entropy_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_plain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_malware: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub girs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CredentialOccurrenceStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_set_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliation_group: Option<AffiliationGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_strength: Option<PasswordStrength>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_length_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_lowercase_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_uppercase_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_numbers_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_punctuation_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_symbols_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_separators_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_other_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_entropy_gte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_plain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_malware: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub girs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CredentialSetStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub girs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub victim: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CredentialSetAccessedUrlStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_set_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessed_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub girs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub victim: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffiliationGroup {
    MyCustomers,
    MyEmployees,
    ThirdParties,
    VipEmails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasswordStrength {
    Excellent,
    Strong,
    Medium,
    Weak,
    Poor,
    NotProvided,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredentialStreamPage {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub credentials: Option<Vec<Credential>>,
}

impl StreamPage for CredentialStreamPage {
    type Item = Credential;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.credentials.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Credential {
    pub id: String,
    pub data: CredDataResponse,
    pub statistics: Option<CredStatisticsResponse>,
    pub classification: Option<ClassificationResponse>,
    pub last_updated_ts: String,
    pub activity: ActivityResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredDataResponse {
    pub credential_login: Option<String>,
    pub credential_domain: Option<String>,
    pub detection_domain: Option<String>,
    pub affiliations: Option<Vec<String>>,
    pub password: Option<CredPasswordResponse>,
    pub credential_sets: Option<Vec<CredCredentialSetResponse>>,
    pub info_stealer: InfoStealerResponseSet,
    pub accessed_url: Option<String>,
    pub credential_set_type: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredPasswordResponse {
    pub complexity: Option<CredPasswordComplexityResponse>,
    pub strength: Option<PasswordStrength>,
    pub id: Option<String>,
    pub password_plain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredPasswordComplexityResponse {
    pub lowercase: Option<i32>,
    pub uppercase: Option<i32>,
    pub numbers: Option<i32>,
    pub symbols: Option<i32>,
    pub punctuation_marks: Option<i32>,
    pub separators: Option<i32>,
    pub other: Option<i32>,
    pub length: Option<i32>,
    pub score: Option<f64>,
    pub weakness: Option<f64>,
    pub entropy: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredCredentialSetResponse {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InfoStealerResponseSet {
    pub malware_family: Option<Vec<String>>,
    pub malware_install_path: Option<Vec<String>>,
    pub screenshot_path: Option<Vec<String>>,
    pub infection_ts: Option<Vec<String>>,
    pub machine_id: Option<Vec<String>>,
    pub pc_name: Option<Vec<String>>,
    pub computer_username: Option<Vec<String>>,
    pub ip: Option<Vec<String>>,
    pub os: Option<Vec<String>>,
    pub antivirus_software: Option<Vec<String>>,
    pub isp: Option<Vec<String>>,
    pub version: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InfoStealerResponseOption {
    pub malware_family: Option<String>,
    pub malware_install_path: Option<String>,
    pub screenshot_path: Option<String>,
    pub infection_ts: Option<String>,
    pub machine_id: Option<String>,
    pub pc_name: Option<String>,
    pub computer_username: Option<String>,
    pub ip: Option<String>,
    pub os: Option<String>,
    pub antivirus_software: Option<String>,
    pub isp: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredStatisticsResponse {
    pub accessed_urls_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClassificationResponse {
    pub girs: Option<Vec<GirsResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GirsResponse {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ActivityResponse {
    pub first_seen_ts: String,
    pub last_seen_ts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredentialOccurrenceStreamPage {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub credential_occurrences: Option<Vec<CredentialOccurrence>>,
}

impl StreamPage for CredentialOccurrenceStreamPage {
    type Item = CredentialOccurrence;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.credential_occurrences.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredentialOccurrence {
    pub id: String,
    pub data: CredentialOccurrenceDataResponse,
    pub classification: Option<ClassificationResponse>,
    pub last_updated_ts: String,
    pub activity: ActivityResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredentialOccurrenceDataResponse {
    pub file_path: Option<String>,
    pub accessed_url: Option<String>,
    pub credential: Option<CredentialOccurrenceCredResponse>,
    pub credential_set: Option<CredCredentialSetResponse>,
    pub info_stealer: InfoStealerResponseOption,
    pub software_name: Option<String>,
    pub credential_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredentialOccurrenceCredResponse {
    pub id: String,
    pub credential_login: Option<String>,
    pub credential_domain: Option<String>,
    pub detection_domain: Option<String>,
    pub affiliations: Option<Vec<String>>,
    pub password: Option<CredPasswordResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredentialSetStreamPage {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub credential_sets: Option<Vec<CredentialSet>>,
}

impl StreamPage for CredentialSetStreamPage {
    type Item = CredentialSet;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.credential_sets.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredentialSet {
    pub id: String,
    pub data: CredSetDataResponse,
    pub statistics: Option<CredSetStatisticsResponse>,
    pub classification: Option<ClassificationResponse>,
    pub last_updated_ts: String,
    pub activity: ActivityResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredSetDataResponse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub record_count: Option<i64>,
    pub disclosure_ts: Option<String>,
    pub breach_ts: Option<String>,
    pub collected_ts: Option<String>,
    pub victims: Option<Vec<VictimResponse>>,
    pub sources: Option<Vec<LinksSource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VictimResponse {
    pub name: Option<String>,
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LinksSource {
    pub title: Option<String>,
    pub links: SourceLinks,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SourceLinks {
    pub verity_api: Option<SourceLink>,
    pub verity_portal: Option<SourceLink>,
    pub external: Option<SourceLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SourceLink {
    pub href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredSetStatisticsResponse {
    pub credentials_count: Option<i64>,
    pub credential_occurrences_count: Option<i64>,
    pub accessed_urls_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredentialSetAccessedUrlStreamPage {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub access_urls: Option<Vec<CredentialSetAccessedUrl>>,
}

impl StreamPage for CredentialSetAccessedUrlStreamPage {
    type Item = CredentialSetAccessedUrl;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.access_urls.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredentialSetAccessedUrl {
    pub id: String,
    pub data: CredSetAccessedUrlDataResponse,
    pub classification: Option<ClassificationResponse>,
    pub last_updated_ts: String,
    pub activity: ActivityResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CredSetAccessedUrlDataResponse {
    pub accessed_url: String,
    pub accessed_domain: Option<String>,
    pub credential_set: Option<CredCredentialSetResponse>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_strength_roundtrip() {
        let json = serde_json::to_string(&PasswordStrength::Excellent).unwrap();
        assert_eq!(json, "\"excellent\"");
        let rt: PasswordStrength = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, PasswordStrength::Excellent));

        let json = serde_json::to_string(&PasswordStrength::NotProvided).unwrap();
        assert_eq!(json, "\"not_provided\"");
        let rt: PasswordStrength = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, PasswordStrength::NotProvided));
    }

    #[test]
    fn test_affiliation_group_roundtrip() {
        let json = serde_json::to_string(&AffiliationGroup::MyCustomers).unwrap();
        assert_eq!(json, "\"my_customers\"");
        let rt: AffiliationGroup = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, AffiliationGroup::MyCustomers));
    }

    #[test]
    fn test_credential_stream_page_deserialize() {
        let json = r#"{"count":2,"cursor_next":"next_tok","credentials":[{"id":"cred-1","data":{"credential_login":"user@example.com","credential_domain":"example.com","affiliations":["aff1"],"password":{"strength":"strong"},"info_stealer":{"malware_family":[],"malware_install_path":[],"screenshot_path":[],"infection_ts":[],"machine_id":[],"pc_name":[],"computer_username":[],"ip":[],"os":[],"antivirus_software":[],"isp":[],"version":[]},"accessed_url":"https://example.com"},"statistics":{"accessed_urls_count":3},"classification":{"girs":[{"path":"/girs/path","name":"GIR Name"}]},"last_updated_ts":"2023-06-01T00:00:00Z","activity":{"first_seen_ts":"2023-01-01T00:00:00Z","last_seen_ts":"2023-06-01T00:00:00Z"}}]}"#;
        let page: CredentialStreamPage = serde_json::from_str(json).unwrap();
        assert_eq!(page.count, 2);
        assert_eq!(page.cursor_next, Some("next_tok".to_string()));
        let creds = page.credentials.unwrap();
        assert_eq!(creds.len(), 1);
        assert_eq!(creds[0].id, "cred-1");
    }

    #[test]
    fn test_credential_set_stream_page_deserialize() {
        let json = r#"{"count":1,"credential_sets":[{"id":"cs-1","data":{"name":"Set Name","description":"A credential set","record_count":100,"disclosure_ts":"2023-01-01T00:00:00Z"},"statistics":{"credentials_count":50,"credential_occurrences_count":200,"accessed_urls_count":10},"classification":{"girs":[{"path":"/path","name":"GIR"}]},"last_updated_ts":"2023-06-01T00:00:00Z","activity":{"first_seen_ts":"2023-01-01T00:00:00Z","last_seen_ts":"2023-06-01T00:00:00Z"}}]}"#;
        let page: CredentialSetStreamPage = serde_json::from_str(json).unwrap();
        assert_eq!(page.count, 1);
        let sets = page.credential_sets.unwrap();
        assert_eq!(sets[0].id, "cs-1");
    }

    #[test]
    fn test_credential_stream_request_serialize() {
        let req = CredentialStreamRequest {
            credential_set_name: Some("test-set".to_string()),
            credential_set_id: None,
            domain: Some("example.com".to_string()),
            affiliation_group: None,
            password_strength: Some(PasswordStrength::Weak),
            password_length_gte: None,
            password_lowercase_gte: None,
            password_uppercase_gte: None,
            password_numbers_gte: None,
            password_punctuation_gte: None,
            password_symbols_gte: None,
            password_separators_gte: None,
            password_other_gte: None,
            password_entropy_gte: None,
            password_plain: None,
            credential_login: None,
            detected_malware: None,
            girs: None,
            from: None,
            until: None,
            last_updated_from: None,
            last_updated_until: None,
            size: Some(50),
            cursor: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"password_strength\":\"weak\""));
        assert!(json.contains("\"domain\":\"example.com\""));
        assert!(!json.contains("credential_set_id"));
    }
}