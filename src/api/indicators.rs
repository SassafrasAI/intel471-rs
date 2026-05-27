use crate::client::Intel471Client;
use crate::error::Result;
use crate::models::indicators::{IndicatorStreamRequest, IndicatorsStream, IntegrationsIndicator};
use crate::pagination::PaginatedStream;

const SERVICE_PATH: &str = "integrations/indicators/v1";

#[derive(Debug, Clone)]
pub struct IndicatorsApi<'a> {
    pub(crate) client: &'a Intel471Client,
}

impl<'a> IndicatorsApi<'a> {
    pub fn stream(&self, request: IndicatorStreamRequest) -> Result<PaginatedStream<'a, IndicatorsStream>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/indicators/stream", url);

        let mut params = vec![];
        if let Some(ref v) = request.type_ {
            let s = serde_json::to_value(v).unwrap();
            params.push(("type", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.threat_type {
            let s = serde_json::to_value(v).unwrap();
            params.push(("threat_type", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.confidence {
            let s = serde_json::to_value(v).unwrap();
            params.push(("confidence", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }
        if let Some(ref v) = request.text_filter {
            params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.malware_id {
            params.push(("malware_id", Some(v.clone())));
        }
        if let Some(ref v) = request.malware_family_id {
            params.push(("malware_family_id", Some(v.clone())));
        }
        if let Some(ref v) = request.malware_family_name {
            params.push(("malware_family_name", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            params.push(("girs", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.from {
            params.push(("from", Some(v.clone())));
        }
        if let Some(ref v) = request.until {
            params.push(("until", Some(v.clone())));
        }

        let query = crate::pagination::build_query(&params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &IndicatorsStream| page.cursor_next.clone(),
        ))
    }

    pub async fn stream_page(&self, request: IndicatorStreamRequest) -> Result<IndicatorsStream> {
        let url = self.client.url(SERVICE_PATH);
        let mut params = vec![];
        if let Some(ref v) = request.type_ {
            let s = serde_json::to_value(v).unwrap();
            params.push(("type", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.threat_type {
            let s = serde_json::to_value(v).unwrap();
            params.push(("threat_type", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.confidence {
            let s = serde_json::to_value(v).unwrap();
            params.push(("confidence", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }
        if let Some(ref v) = request.text_filter {
            params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.malware_id {
            params.push(("malware_id", Some(v.clone())));
        }
        if let Some(ref v) = request.malware_family_id {
            params.push(("malware_family_id", Some(v.clone())));
        }
        if let Some(ref v) = request.malware_family_name {
            params.push(("malware_family_name", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            params.push(("girs", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.from {
            params.push(("from", Some(v.clone())));
        }
        if let Some(ref v) = request.until {
            params.push(("until", Some(v.clone())));
        }

        let query = crate::pagination::build_query(&params);
        let full_url = if query.is_empty() {
            format!("{}/indicators/stream", url)
        } else {
            format!("{}/indicators/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<IntegrationsIndicator> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/indicators/{}", url, id);
        self.client.get(&full_url).await
    }
}