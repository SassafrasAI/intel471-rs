use serde::{Deserialize, Serialize};

use crate::models::common::{Activity, Report};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct EntityStreamRequest {
    pub entity: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<EntityType>,
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
pub enum EntityType {
    #[serde(rename = "AIM")]
    AIM,
    #[serde(rename = "ActorDomain")]
    ActorDomain,
    #[serde(rename = "BitcoinAddress")]
    BitcoinAddress,
    #[serde(rename = "BitcoinTransactionID")]
    BitcoinTransactionID,
    #[serde(rename = "CveID")]
    CveID,
    #[serde(rename = "OtherCryptoCurrencies")]
    OtherCryptoCurrencies,
    #[serde(rename = "EmailAddress")]
    EmailAddress,
    #[serde(rename = "Facebook")]
    Facebook,
    #[serde(rename = "Handle")]
    Handle,
    #[serde(rename = "ICQ")]
    ICQ,
    #[serde(rename = "IPv4Prefix")]
    IPv4Prefix,
    #[serde(rename = "IPv6Prefix")]
    IPv6Prefix,
    #[serde(rename = "Jabber")]
    Jabber,
    #[serde(rename = "MSN")]
    MSN,
    #[serde(rename = "MalwareFamily")]
    MalwareFamily,
    #[serde(rename = "MobileMalwareFamily")]
    MobileMalwareFamily,
    #[serde(rename = "PerfectMoneyID")]
    PerfectMoneyID,
    #[serde(rename = "Phone")]
    Phone,
    #[serde(rename = "QQ")]
    QQ,
    #[serde(rename = "QiwiWallet")]
    QiwiWallet,
    #[serde(rename = "Skype")]
    Skype,
    #[serde(rename = "Twitter")]
    Twitter,
    #[serde(rename = "X")]
    X,
    #[serde(rename = "URL")]
    URL,
    #[serde(rename = "VK")]
    VK,
    #[serde(rename = "WebMoneyID")]
    WebMoneyID,
    #[serde(rename = "WebMoneyPurse")]
    WebMoneyPurse,
    #[serde(rename = "YahooIM")]
    YahooIM,
    #[serde(rename = "YandexMoney")]
    YandexMoney,
    #[serde(rename = "Telegram")]
    Telegram,
    #[serde(rename = "Tag")]
    Tag,
    #[serde(rename = "Tox")]
    Tox,
    #[serde(rename = "PasswordHash")]
    PasswordHash,
    #[serde(rename = "Password")]
    Password,
    #[serde(rename = "PGPKey")]
    PGPKey,
    #[serde(rename = "PGPKeyID")]
    PGPKeyID,
    #[serde(rename = "ActorOtherWebsite")]
    ActorOtherWebsite,
    #[serde(rename = "Odnoklassniki")]
    Odnoklassniki,
    #[serde(rename = "MoiMir")]
    MoiMir,
    #[serde(rename = "Instagram")]
    Instagram,
    #[serde(rename = "LinkedIn")]
    LinkedIn,
    #[serde(rename = "GitHub")]
    GitHub,
    #[serde(rename = "Discord")]
    Discord,
    #[serde(rename = "WeChat")]
    WeChat,
    #[serde(rename = "Wickr")]
    Wickr,
    #[serde(rename = "Signal")]
    Signal,
    #[serde(rename = "IPAddress")]
    IPAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EntityStreamPage {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub entities: Option<Vec<Entity>>,
}

impl crate::models::common::StreamPage for EntityStreamPage {
    type Item = Entity;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.entities.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Entity {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: EntityType,
    pub value: String,
    pub activity: Activity,
    pub report: Option<Report>,
    pub actor: Option<ActorObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ActorObject {
    pub id: String,
    pub handles: Option<Vec<String>>,
    pub forum: Option<ForumObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ForumObject {
    pub id: String,
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_type_roundtrip() {
        let json = serde_json::to_string(&EntityType::BitcoinAddress).unwrap();
        assert_eq!(json, "\"BitcoinAddress\"");
        let rt: EntityType = serde_json::from_str(&json).unwrap();
        assert!(matches!(rt, EntityType::BitcoinAddress));
    }

    #[test]
    fn test_entity_stream_page_deserialize() {
        let json = r#"{"count":2,"cursor_next":"abc123","entities":[{"id":"e1","type":"Twitter","value":"@test","activity":{"first_seen_ts":"2023-01-01","last_seen_ts":"2023-06-01"}}]}"#;
        let page: EntityStreamPage = serde_json::from_str(json).unwrap();
        assert_eq!(page.count, 2);
        assert_eq!(page.cursor_next, Some("abc123".to_string()));
        assert_eq!(page.entities.unwrap().len(), 1);
    }

    #[test]
    fn test_entity_stream_request_serialize() {
        let req = EntityStreamRequest {
            entity: "intel.com".to_string(),
            type_: Some(EntityType::URL),
            from: Some(1627776000000),
            until: None,
            size: Some(100),
            cursor: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"entity\":\"intel.com\""));
        assert!(json.contains("\"type\":\"URL\""));
        assert!(!json.contains("until"));
        assert!(!json.contains("cursor"));
    }
}