use serde::{Deserialize, Serialize};

use crate::models::common::StreamPage;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct AlertsStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watcher_group_ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watcher_ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_trashed_included: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamingAlertsResponse {
    pub alerts: Vec<StreamingWatcherAlert>,
    pub count: i64,
    pub cursor_next: Option<String>,
}

impl StreamPage for StreamingAlertsResponse {
    type Item = StreamingWatcherAlert;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { &self.alerts }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamingWatcherAlert {
    pub creation_ts: Option<String>,
    pub highlights: Option<Vec<Highlight>>,
    pub id: Option<i64>,
    pub is_trashed: Option<bool>,
    pub links: Option<WatchersLinks>,
    pub source_id: Option<String>,
    pub source_type: Option<String>,
    pub status: Option<String>,
    pub watcher_group_id: Option<i64>,
    pub watcher_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Highlight {
    pub field_name: String,
    pub snippets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WatchersLinks {
    pub verity_api: Option<Href>,
    pub verity_portal: Option<Href>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Href {
    pub href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertStatus {
    Generated,
    NeedsAction,
    InProgress,
    Completed,
    FalsePositive,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct WatcherGroupsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watcher_group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watcher_group_type: Option<WatcherGroupType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetWatcherGroupResponseWrapper {
    pub count: i64,
    pub watchers_groups: Vec<GetWatcherGroupResponse>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetWatcherGroupResponse {
    pub created_by: Option<String>,
    pub creation_ts: Option<String>,
    pub description: Option<String>,
    pub id: Option<i64>,
    pub is_editable: Option<bool>,
    pub is_global: Option<bool>,
    pub is_muted: Option<bool>,
    pub is_subscribed: Option<bool>,
    pub last_updated_ts: Option<String>,
    pub name: Option<String>,
    pub notification_settings: Option<Vec<NotificationSettingsResponse>>,
    pub organisation_id: Option<String>,
    pub owner_user_id: Option<String>,
    pub sharing_settings: Option<Vec<ShareSettingsResponse>>,
    pub updated_by: Option<String>,
    pub watcher_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WatcherGroupType {
    Editable,
    Owned,
    Subscribed,
    Shared,
    #[serde(rename = "i471_global")]
    I471Global,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotificationSettingsResponse {
    pub created: Option<String>,
    pub notification_channel: Option<String>,
    pub notification_preference_type: Option<NotificationPreferenceType>,
    pub notify_at_utc_hour: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationPreferenceType {
    #[serde(rename = "UTC_HOUR")]
    UtcHour,
    #[serde(rename = "IMMEDIATELY")]
    Immediately,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShareSettingsResponse {
    pub created: Option<String>,
    pub organisation_id: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct WatchersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watcher_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watcher_group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dsl_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watcher_group_type: Option<WatcherGroupType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetWatcherResponseWrapper {
    pub count: i64,
    pub watchers: Vec<GetWatcherResponse>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetWatcherResponse {
    pub created_by: Option<String>,
    pub creation_ts: Option<String>,
    pub data_sets: Option<Vec<String>>,
    pub description: Option<String>,
    pub dsl_query: Option<String>,
    pub id: Option<i64>,
    pub is_muted: Option<bool>,
    pub last_updated_ts: Option<String>,
    pub name: Option<String>,
    pub notification_settings: Option<Vec<NotificationSettingsResponse>>,
    pub query_fields: Option<Vec<String>>,
    pub updated_by: Option<String>,
    pub watcher_group_id: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_status_roundtrip() {
        let json = serde_json::to_string(&AlertStatus::Generated).unwrap();
        assert_eq!(json, "\"generated\"");
        let rt: AlertStatus = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, AlertStatus::Generated));

        let json = serde_json::to_string(&AlertStatus::NeedsAction).unwrap();
        assert_eq!(json, "\"needs_action\"");
        let rt: AlertStatus = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, AlertStatus::NeedsAction));

        let json = serde_json::to_string(&AlertStatus::InProgress).unwrap();
        assert_eq!(json, "\"in_progress\"");
        let rt: AlertStatus = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, AlertStatus::InProgress));
    }

    #[test]
    fn test_watcher_group_type_roundtrip() {
        let json = serde_json::to_string(&WatcherGroupType::Editable).unwrap();
        assert_eq!(json, "\"editable\"");
        let rt: WatcherGroupType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, WatcherGroupType::Editable));

        let json = serde_json::to_string(&WatcherGroupType::I471Global).unwrap();
        assert_eq!(json, "\"i471_global\"");
        let rt: WatcherGroupType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, WatcherGroupType::I471Global));
    }

    #[test]
    fn test_notification_preference_type_explicit_rename_roundtrip() {
        let json = serde_json::to_string(&NotificationPreferenceType::UtcHour).unwrap();
        assert_eq!(json, "\"UTC_HOUR\"");
        let rt: NotificationPreferenceType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, NotificationPreferenceType::UtcHour));

        let json = serde_json::to_string(&NotificationPreferenceType::Immediately).unwrap();
        assert_eq!(json, "\"IMMEDIATELY\"");
        let rt: NotificationPreferenceType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, NotificationPreferenceType::Immediately));
    }

    #[test]
    fn test_streaming_alerts_response_deserialize() {
        let json = r#"{"alerts":[{"creation_ts":"2023-01-01T00:00:00Z","highlights":[{"field_name":"message","snippets":["match text"]}],"id":12345,"is_trashed":false,"source_id":"src-1","source_type":"forum_post","status":"generated","watcher_group_id":1,"watcher_id":2}],"count":1}"#;
        let resp: StreamingAlertsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.count, 1);
        assert_eq!(resp.alerts.len(), 1);
        assert_eq!(resp.alerts[0].id, Some(12345));
    }

    #[test]
    fn test_get_watcher_group_response_deserialize() {
        let json = r#"{"count":1,"watchers_groups":[{"id":1,"name":"My Watcher Group","description":"Test group","is_editable":true,"is_global":false,"is_muted":false,"is_subscribed":true,"creation_ts":"2023-01-01T00:00:00Z","last_updated_ts":"2023-06-01T00:00:00Z"}]}"#;
        let resp: GetWatcherGroupResponseWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(resp.count, 1);
        assert_eq!(resp.watchers_groups.len(), 1);
        assert_eq!(resp.watchers_groups[0].name, Some("My Watcher Group".to_string()));
    }

    #[test]
    fn test_streaming_watcher_alert_deserialize() {
        let json = r#"{"creation_ts":"2023-06-15T12:00:00Z","highlights":[{"field_name":"title","snippets":["critical"]}],"id":99,"is_trashed":false,"links":{"verity_api":{"href":"https://api.example.com/alerts/99"},"verity_portal":{"href":"https://portal.example.com/alerts/99"}},"source_id":"source-abc","source_type":"forum_post","status":"needs_action","watcher_group_id":5,"watcher_id":10}"#;
        let alert: StreamingWatcherAlert = serde_json::from_str(json).unwrap();
        assert_eq!(alert.id, Some(99));
        assert_eq!(alert.source_type, Some("forum_post".to_string()));
        assert!(alert.links.is_some());
        let links = alert.links.unwrap();
        assert_eq!(links.verity_api.unwrap().href, "https://api.example.com/alerts/99");
    }
}