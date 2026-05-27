use crate::client::Intel471Client;
use crate::error::Result;
use crate::models::entities::{EntityStreamPage, EntityStreamRequest};
use crate::pagination::PaginatedStream;

const SERVICE_PATH: &str = "integrations/entities/v1";

#[derive(Debug, Clone)]
pub struct EntitiesApi<'a> {
    pub(crate) client: &'a Intel471Client,
}

impl<'a> EntitiesApi<'a> {
    pub fn stream(&self, request: EntityStreamRequest) -> Result<PaginatedStream<'a, EntityStreamPage>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/entities/stream", url);

        let mut params = vec![("entity", Some(request.entity.clone()))];
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
        let _full_url = if query.is_empty() {
            base_url.clone()
        } else {
            format!("{}?{}", base_url, query)
        };

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &EntityStreamPage| page.cursor_next.clone(),
        ))
    }

    pub async fn stream_page(&self, request: EntityStreamRequest) -> Result<EntityStreamPage> {
        let url = self.client.url(SERVICE_PATH);
        let mut params = vec![("entity", Some(request.entity.clone()))];
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
        let full_url = if query.is_empty() {
            format!("{}/entities/stream", url)
        } else {
            format!("{}/entities/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }
}