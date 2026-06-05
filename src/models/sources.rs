use serde::{Deserialize, Serialize};

use crate::models::common::{Activity, StreamPage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatServerTypeStream {
    #[serde(rename = "telegram")]
    Telegram,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "irc")]
    IRC,
    #[serde(rename = "whats_app")]
    WhatsApp,
    #[serde(rename = "qq")]
    QQ,
    #[serde(rename = "icq")]
    ICQ,
    #[serde(rename = "matrix")]
    Matrix,
    #[serde(rename = "signal")]
    Signal,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageType {
    Large,
    Original,
    Thumb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentClassification {
    CSAM,
    Safe,
    Unknown,
    Unsafe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingStatus {
    Converted,
    Failed,
    Pending,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportingStatus {
    None,
    Removed,
    Reported,
    Validated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TranslationStatus {
    WaitingForTranslation,
    InProgress,
    Done,
    Rejected,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct DataLeakSitePostsStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ForumsPostsStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ForumsPrivateMessagesStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatMessagesStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[serde(rename = "server_type", skip_serializing_if = "Option::is_none")]
    pub server_type: Option<ChatServerTypeStream>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SourcesLinks {
    pub external: Option<Link>,
    pub verity_api: Option<Link>,
    pub verity_portal: Option<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Link {
    pub href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthorActor {
    pub id: String,
    pub user_name: String,
    pub user_name_id: String,
    pub historical_usernames: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachmentData {
    pub id: String,
    pub source_entity_id: String,
    pub file_name: String,
    pub file_hash: String,
    pub file_size: i64,
    pub mime_type: String,
    pub processing_status: ProcessingStatus,
    pub reporting_status: ReportingStatus,
    pub thumbnail_available: bool,
    pub webp_available: bool,
    pub file_available: bool,
    pub is_not_image: bool,
    pub attachment_highlights: Option<Vec<String>>,
    pub attachment_original_url: Option<String>,
    pub classification: Option<AttachmentClassification>,
    pub dimensions: Option<String>,
    pub file_url: Option<String>,
    pub highlights: Option<Vec<Highlight>>,
    pub original_height: Option<i32>,
    pub original_width: Option<i32>,
    pub recognized_logos: Option<String>,
    pub recognized_text: Option<String>,
    pub thumbnail_height: Option<i32>,
    pub thumbnail_width: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Highlight {
    pub field: String,
    pub snippets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EntityItem {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataLeakSitePostsStreamingPage {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub posts: Option<Vec<DataLeakSitePostItem>>,
}

impl StreamPage for DataLeakSitePostsStreamingPage {
    type Item = DataLeakSitePostItem;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.posts.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataLeakSitePostItem {
    pub post: DataLeakSitePost,
    pub thread: DataLeakSitePostThread,
    pub website: DataLeakSitePostWebsite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataLeakSitePost {
    pub title: String,
    pub message: String,
    pub creation_ts: String,
    pub last_updated_ts: String,
    pub attachments: Option<Vec<AttachmentData>>,
    pub file_listing: Option<DataLeakSiteFileListingUrl>,
    pub inactive_since: Option<String>,
    pub is_inactive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataLeakSiteFileListingUrl {
    pub download_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataLeakSitePostThread {
    pub id: String,
    pub title: String,
    pub links: SourcesLinks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataLeakSitePostWebsite {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ForumsPostsStreamingPage {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub posts: Option<Vec<PostDetails>>,
}

impl StreamPage for ForumsPostsStreamingPage {
    type Item = PostDetails;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.posts.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostDetails {
    pub post: PostResponse,
    pub forum: Option<ForumsResponse>,
    pub sub_forum: Option<SubForumResponse>,
    pub thread: Option<ThreadResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostResponse {
    pub id: String,
    pub message: String,
    pub html: String,
    pub creation_ts: String,
    pub last_updated_ts: String,
    pub author: Option<AuthorActor>,
    pub attachments: Option<Vec<AttachmentData>>,
    pub entities: Option<Vec<EntityItem>>,
    pub translated_message: Option<String>,
    pub translation_status: Option<TranslationStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ForumsResponse {
    pub id: String,
    pub title: String,
    pub activity: Activity,
    pub description: Option<String>,
    pub links: Option<SourcesLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubForumResponse {
    pub id: String,
    pub title: String,
    pub activity: Activity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ThreadResponse {
    pub id: String,
    pub post_count: i64,
    pub activity: Activity,
    pub links: Option<SourcesLinks>,
    pub topic: Option<String>,
    pub topic_original: Option<String>,
    pub translation_status: Option<TranslationStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ForumsPrivateMessagesStreamingPage {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub private_messages: Option<Vec<PrivateMessageDetails>>,
}

impl StreamPage for ForumsPrivateMessagesStreamingPage {
    type Item = PrivateMessageDetails;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.private_messages.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivateMessageDetails {
    pub private_message: PrivateMessageResponse,
    pub author: Option<AuthorActor>,
    pub recipient: Option<AuthorActor>,
    pub forum: Option<ForumsResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivateMessageResponse {
    pub id: String,
    pub message: String,
    pub creation_ts: String,
    pub conversation_id: Option<String>,
    pub subject: Option<String>,
    pub translated_message: Option<String>,
    pub translated_subject: Option<String>,
    pub translation_status: Option<TranslationStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatMessagesStreamingPage {
    pub count: i64,
    pub cursor_next: Option<String>,
    pub messages: Option<Vec<ChatRoomMessage>>,
}

impl StreamPage for ChatMessagesStreamingPage {
    type Item = ChatRoomMessage;
    fn count(&self) -> i64 { self.count }
    fn cursor_next(&self) -> Option<String> { self.cursor_next.clone() }
    fn items(&self) -> &[Self::Item] { self.messages.as_deref().unwrap_or(&[]) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatRoomMessage {
    pub message: ChatMessageStream,
    pub chat_room: Option<RoomStream>,
    pub server: Option<ServerStream>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatMessageStream {
    pub id: String,
    pub text: String,
    pub html: String,
    pub creation_ts: String,
    pub author: Option<AuthorActor>,
    pub attachments: Option<Vec<AttachmentData>>,
    pub links: Option<SourcesLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RoomStream {
    pub id: String,
    pub name: String,
    pub message_count: i64,
    pub activity: Activity,
    pub channel_id: String,
    pub description: Option<String>,
    pub links: Option<SourcesLinks>,
    pub usernames: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ServerStream {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub links: Option<SourcesLinks>,
}