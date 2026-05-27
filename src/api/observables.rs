use crate::client::Intel471Client;
use crate::error::Result;
use crate::models::observables::{ObservableStreamPage, ObservableStreamRequest};
use crate::pagination::PaginatedStream;

const SERVICE_PATH: &str = "integrations/observables/v1";

#[derive(Debug, Clone)]
pub struct ObservablesApi<'a> {
    pub(crate) client: &'a Intel471Client,
}

impl<'a> ObservablesApi<'a> {
    pub fn stream(&self, request: ObservableStreamRequest) -> Result<PaginatedStream<'a, ObservableStreamPage>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/observables/stream", url);

        let mut params = vec![("observable", Some(request.observable.clone()))];
        if let Some(ref v) = request.type_ {
            let s = serde_json::to_value(v).unwrap();
            params.push(("type", Some(s.as_str().unwrap().to_string())));
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

        let query = crate::pagination::build_query(&params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &ObservableStreamPage| page.cursor_next.clone(),
        ))
    }

    pub async fn stream_page(&self, request: ObservableStreamRequest) -> Result<ObservableStreamPage> {
        let url = self.client.url(SERVICE_PATH);
        let mut params = vec![("observable", Some(request.observable.clone()))];
        if let Some(ref v) = request.type_ {
            let s = serde_json::to_value(v).unwrap();
            params.push(("type", Some(s.as_str().unwrap().to_string())));
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

        let query = crate::pagination::build_query(&params);
        let full_url = format!("{}/observables/stream?{}", url, query);
        self.client.get(&full_url).await
    }
}