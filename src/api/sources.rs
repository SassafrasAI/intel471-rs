use crate::client::Intel471Client;
use crate::error::Result;
use crate::models::sources::{
    ChatMessagesStreamRequest, ChatMessagesStreamingPage,
    DataLeakSitePostsStreamRequest, DataLeakSitePostsStreamingPage,
    ForumsPostsStreamRequest, ForumsPostsStreamingPage, ForumsPrivateMessagesStreamRequest,
    ForumsPrivateMessagesStreamingPage, ImageType, PostDetails, PrivateMessageDetails,
    ChatRoomMessage,
};
use crate::pagination::{PaginatedStream, build_query};

const SERVICE_PATH: &str = "integrations/sources/v1";

#[derive(Debug, Clone)]
pub struct SourcesApi<'a> {
    pub(crate) client: &'a Intel471Client,
}

impl<'a> SourcesApi<'a> {
    pub fn data_leak_site_posts_stream(
        &self,
        request: DataLeakSitePostsStreamRequest,
    ) -> Result<PaginatedStream<'a, DataLeakSitePostsStreamingPage>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/data-leak-sites/posts/stream", url);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.website_id {
            params.push(("website_id", Some(v.clone())));
        }
        if let Some(ref v) = request.thread_id {
            params.push(("thread_id", Some(v.clone())));
        }
        if let Some(ref v) = request.text_filter {
            params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.from {
            params.push(("from", Some(v.clone())));
        }
        if let Some(ref v) = request.until {
            params.push(("until", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }

        let query = build_query(&params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &DataLeakSitePostsStreamingPage| page.cursor_next.clone(),
        ))
    }

    pub async fn data_leak_site_posts_stream_page(
        &self,
        request: DataLeakSitePostsStreamRequest,
    ) -> Result<DataLeakSitePostsStreamingPage> {
        let url = self.client.url(SERVICE_PATH);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.website_id {
            params.push(("website_id", Some(v.clone())));
        }
        if let Some(ref v) = request.thread_id {
            params.push(("thread_id", Some(v.clone())));
        }
        if let Some(ref v) = request.text_filter {
            params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.from {
            params.push(("from", Some(v.clone())));
        }
        if let Some(ref v) = request.until {
            params.push(("until", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }

        let query = build_query(&params);
        let full_url = if query.is_empty() {
            format!("{}/data-leak-sites/posts/stream", url)
        } else {
            format!("{}/data-leak-sites/posts/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn get_data_leak_site_file_listing(&self, id: &str) -> Result<reqwest::Response> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/data-leak-sites/file-listings/{}", url, id);
        self.client.get_raw(&full_url).await
    }

    pub fn forums_posts_stream(
        &self,
        request: ForumsPostsStreamRequest,
    ) -> Result<PaginatedStream<'a, ForumsPostsStreamingPage>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/forums/posts/stream", url);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.thread_id {
            params.push(("thread_id", Some(v.clone())));
        }
        if let Some(ref v) = request.author {
            params.push(("author", Some(v.clone())));
        }
        if let Some(ref v) = request.author_id {
            params.push(("author_id", Some(v.clone())));
        }
        if let Some(ref v) = request.forum_title {
            params.push(("forum_title", Some(v.clone())));
        }
        if let Some(ref v) = request.text_filter {
            params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.from {
            params.push(("from", Some(v.clone())));
        }
        if let Some(ref v) = request.until {
            params.push(("until", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }

        let query = build_query(&params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &ForumsPostsStreamingPage| page.cursor_next.clone(),
        ))
    }

    pub async fn forums_posts_stream_page(
        &self,
        request: ForumsPostsStreamRequest,
    ) -> Result<ForumsPostsStreamingPage> {
        let url = self.client.url(SERVICE_PATH);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.thread_id {
            params.push(("thread_id", Some(v.clone())));
        }
        if let Some(ref v) = request.author {
            params.push(("author", Some(v.clone())));
        }
        if let Some(ref v) = request.author_id {
            params.push(("author_id", Some(v.clone())));
        }
        if let Some(ref v) = request.forum_title {
            params.push(("forum_title", Some(v.clone())));
        }
        if let Some(ref v) = request.text_filter {
            params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.from {
            params.push(("from", Some(v.clone())));
        }
        if let Some(ref v) = request.until {
            params.push(("until", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }

        let query = build_query(&params);
        let full_url = if query.is_empty() {
            format!("{}/forums/posts/stream", url)
        } else {
            format!("{}/forums/posts/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn get_forum_post(&self, post_id: &str) -> Result<PostDetails> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/forums/posts/{}", url, post_id);
        self.client.get(&full_url).await
    }

    pub fn forums_private_messages_stream(
        &self,
        request: ForumsPrivateMessagesStreamRequest,
    ) -> Result<PaginatedStream<'a, ForumsPrivateMessagesStreamingPage>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/forums/private-messages/stream", url);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.text_filter {
            params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.author {
            params.push(("author", Some(v.clone())));
        }
        if let Some(ref v) = request.author_id {
            params.push(("author_id", Some(v.clone())));
        }
        if let Some(ref v) = request.forum_title {
            params.push(("forum_title", Some(v.clone())));
        }
        if let Some(ref v) = request.subject {
            params.push(("subject", Some(v.clone())));
        }
        if let Some(ref v) = request.recipient {
            params.push(("recipient", Some(v.clone())));
        }
        if let Some(ref v) = request.recipient_id {
            params.push(("recipient_id", Some(v.clone())));
        }
        if let Some(ref v) = request.forum_id {
            params.push(("forum_id", Some(v.clone())));
        }
        if let Some(v) = request.from {
            params.push(("from", Some(v.to_string())));
        }
        if let Some(v) = request.until {
            params.push(("until", Some(v.to_string())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }

        let query = build_query(&params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &ForumsPrivateMessagesStreamingPage| page.cursor_next.clone(),
        ))
    }

    pub async fn forums_private_messages_stream_page(
        &self,
        request: ForumsPrivateMessagesStreamRequest,
    ) -> Result<ForumsPrivateMessagesStreamingPage> {
        let url = self.client.url(SERVICE_PATH);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.text_filter {
            params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.author {
            params.push(("author", Some(v.clone())));
        }
        if let Some(ref v) = request.author_id {
            params.push(("author_id", Some(v.clone())));
        }
        if let Some(ref v) = request.forum_title {
            params.push(("forum_title", Some(v.clone())));
        }
        if let Some(ref v) = request.subject {
            params.push(("subject", Some(v.clone())));
        }
        if let Some(ref v) = request.recipient {
            params.push(("recipient", Some(v.clone())));
        }
        if let Some(ref v) = request.recipient_id {
            params.push(("recipient_id", Some(v.clone())));
        }
        if let Some(ref v) = request.forum_id {
            params.push(("forum_id", Some(v.clone())));
        }
        if let Some(v) = request.from {
            params.push(("from", Some(v.to_string())));
        }
        if let Some(v) = request.until {
            params.push(("until", Some(v.to_string())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }

        let query = build_query(&params);
        let full_url = if query.is_empty() {
            format!("{}/forums/private-messages/stream", url)
        } else {
            format!("{}/forums/private-messages/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn get_forum_private_message(&self, private_message_id: &str) -> Result<PrivateMessageDetails> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/forums/private-messages/{}", url, private_message_id);
        self.client.get(&full_url).await
    }

    pub async fn get_image(&self, image_type: ImageType, hash: &str, name: &str) -> Result<reqwest::Response> {
        let url = self.client.url(SERVICE_PATH);
        let type_str = serde_json::to_value(&image_type).unwrap();
        let full_url = format!(
            "{}/images/{}/{}/{}",
            url,
            type_str.as_str().unwrap(),
            hash,
            name
        );
        self.client.get_raw(&full_url).await
    }

    pub fn chat_messages_stream(
        &self,
        request: ChatMessagesStreamRequest,
    ) -> Result<PaginatedStream<'a, ChatMessagesStreamingPage>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/messaging-services/messages/stream", url);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.text_filter {
            params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.author {
            params.push(("author", Some(v.clone())));
        }
        if let Some(ref v) = request.author_id {
            params.push(("author_id", Some(v.clone())));
        }
        if let Some(ref v) = request.server_type {
            let s = serde_json::to_value(v).unwrap();
            params.push(("server_type", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.server_id {
            params.push(("server_id", Some(v.clone())));
        }
        if let Some(ref v) = request.room_id {
            params.push(("room_id", Some(v.clone())));
        }
        if let Some(ref v) = request.from {
            params.push(("from", Some(v.clone())));
        }
        if let Some(ref v) = request.until {
            params.push(("until", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }

        let query = build_query(&params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &ChatMessagesStreamingPage| page.cursor_next.clone(),
        ))
    }

    pub async fn chat_messages_stream_page(
        &self,
        request: ChatMessagesStreamRequest,
    ) -> Result<ChatMessagesStreamingPage> {
        let url = self.client.url(SERVICE_PATH);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.text_filter {
            params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.author {
            params.push(("author", Some(v.clone())));
        }
        if let Some(ref v) = request.author_id {
            params.push(("author_id", Some(v.clone())));
        }
        if let Some(ref v) = request.server_type {
            let s = serde_json::to_value(v).unwrap();
            params.push(("server_type", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.server_id {
            params.push(("server_id", Some(v.clone())));
        }
        if let Some(ref v) = request.room_id {
            params.push(("room_id", Some(v.clone())));
        }
        if let Some(ref v) = request.from {
            params.push(("from", Some(v.clone())));
        }
        if let Some(ref v) = request.until {
            params.push(("until", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }

        let query = build_query(&params);
        let full_url = if query.is_empty() {
            format!("{}/messaging-services/messages/stream", url)
        } else {
            format!("{}/messaging-services/messages/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn get_chat_message(&self, message_id: &str) -> Result<ChatRoomMessage> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/messaging-services/messages/{}", url, message_id);
        self.client.get(&full_url).await
    }
}