use serde::{Deserialize, Serialize};

use crate::models::common::{Activity, ChatServerType, Report};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ActorStreamRequest {
    pub actor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<ChatServerType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ActorStreamPage {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub actors: Vec<Actor>,
}

impl crate::models::common::StreamPage for ActorStreamPage {
    type Item = Actor;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { &self.actors }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Actor {
    pub id: String,
    pub activity: Activity,
    pub handles: Option<Vec<String>>,
    pub forum: Option<Forum>,
    pub instant_message_server: Option<ImServer>,
    pub report: Option<Report>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Forum {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImServer {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ChatServerType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor_stream_page_deserialize() {
        let json = r#"{"count":1,"cursor_next":"next_page_token","actors":[{"id":"actor-123","activity":{"first_seen_ts":"2023-01-01T00:00:00Z","last_seen_ts":"2023-06-01T00:00:00Z"},"handles":["handle1"],"instant_message_server":{"id":"im-123","type":"Telegram"}}]}"#;
        let page: ActorStreamPage = serde_json::from_str(json).unwrap();
        assert_eq!(page.count, 1);
        assert_eq!(page.cursor_next, Some("next_page_token".to_string()));
        assert_eq!(page.actors.len(), 1);
        assert_eq!(page.actors[0].id, "actor-123");
    }

    #[test]
    fn test_actor_stream_request_serialize() {
        let req = ActorStreamRequest {
            actor: "test_actor".to_string(),
            forum: None,
            from: Some(1627776000000),
            until: None,
            server_type: Some(ChatServerType::Telegram),
            size: None,
            cursor: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"actor\":\"test_actor\""));
        assert!(json.contains("\"server_type\":\"Telegram\""));
        assert!(!json.contains("forum"));
        assert!(!json.contains("cursor"));
    }
}