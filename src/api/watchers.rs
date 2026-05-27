use crate::client::Intel471Client;
use crate::error::Result;
use crate::models::watchers::{
    AlertsStreamRequest, AlertStatus, GetWatcherGroupResponseWrapper, GetWatcherResponseWrapper,
    StreamingAlertsResponse, WatcherGroupsRequest, WatchersRequest,
};
use crate::pagination::{PaginatedStream, build_query};

const SERVICE_PATH: &str = "integrations/watchers/v1";

#[derive(Debug, Clone)]
pub struct WatchersApi<'a> {
    pub(crate) client: &'a Intel471Client,
}

impl<'a> WatchersApi<'a> {
    pub fn alerts_stream(
        &self,
        request: AlertsStreamRequest,
    ) -> Result<PaginatedStream<'a, StreamingAlertsResponse>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/alerts/stream", url);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(v) = request.from {
            params.push(("from", Some(v.to_string())));
        }
        if let Some(v) = request.until {
            params.push(("until", Some(v.to_string())));
        }
        if let Some(ref v) = request.watcher_group_ids {
            params.push(("watcher_group_ids", Some(v.clone())));
        }
        if let Some(ref v) = request.watcher_ids {
            params.push(("watcher_ids", Some(v.clone())));
        }
        if let Some(ref v) = request.statuses {
            params.push(("statuses", Some(v.clone())));
        }
        if let Some(v) = request.is_trashed_included {
            params.push(("is_trashed_included", Some(v.to_string())));
        }

        let query = build_query(&params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &StreamingAlertsResponse| page.cursor_next.clone(),
        ))
    }

    pub async fn alerts_stream_page(
        &self,
        request: AlertsStreamRequest,
    ) -> Result<StreamingAlertsResponse> {
        let url = self.client.url(SERVICE_PATH);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(v) = request.from {
            params.push(("from", Some(v.to_string())));
        }
        if let Some(v) = request.until {
            params.push(("until", Some(v.to_string())));
        }
        if let Some(ref v) = request.watcher_group_ids {
            params.push(("watcher_group_ids", Some(v.clone())));
        }
        if let Some(ref v) = request.watcher_ids {
            params.push(("watcher_ids", Some(v.clone())));
        }
        if let Some(ref v) = request.statuses {
            params.push(("statuses", Some(v.clone())));
        }
        if let Some(v) = request.is_trashed_included {
            params.push(("is_trashed_included", Some(v.to_string())));
        }

        let query = build_query(&params);
        let full_url = if query.is_empty() {
            format!("{}/alerts/stream", url)
        } else {
            format!("{}/alerts/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn update_alert_status(&self, id: i64, status: AlertStatus) -> Result<()> {
        let url = self.client.url(SERVICE_PATH);
        let status_str = serde_json::to_value(&status)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let full_url = format!("{}/alerts/{}/{}", url, id, status_str);
        self.client.put_void(&full_url, &serde_json::json!({})).await
    }

    pub async fn watcher_groups(
        &self,
        request: WatcherGroupsRequest,
    ) -> Result<GetWatcherGroupResponseWrapper> {
        let url = self.client.url(SERVICE_PATH);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(v) = request.watcher_group_id {
            params.push(("watcher_group_id", Some(v.to_string())));
        }
        if let Some(ref v) = request.watcher_group_type {
            let s = serde_json::to_value(v).unwrap();
            params.push(("watcher_group_type", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.name {
            params.push(("name", Some(v.clone())));
        }

        let query = build_query(&params);
        let full_url = if query.is_empty() {
            format!("{}/watcher-groups", url)
        } else {
            format!("{}/watcher-groups?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn watchers(
        &self,
        request: WatchersRequest,
    ) -> Result<GetWatcherResponseWrapper> {
        let url = self.client.url(SERVICE_PATH);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(v) = request.watcher_id {
            params.push(("watcher_id", Some(v.to_string())));
        }
        if let Some(v) = request.watcher_group_id {
            params.push(("watcher_group_id", Some(v.to_string())));
        }
        if let Some(ref v) = request.dsl_query {
            params.push(("dsl_query", Some(v.clone())));
        }
        if let Some(ref v) = request.watcher_group_type {
            let s = serde_json::to_value(v).unwrap();
            params.push(("watcher_group_type", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.name {
            params.push(("name", Some(v.clone())));
        }

        let query = build_query(&params);
        let full_url = if query.is_empty() {
            format!("{}/watchers", url)
        } else {
            format!("{}/watchers?{}", url, query)
        };

        self.client.get(&full_url).await
    }
}