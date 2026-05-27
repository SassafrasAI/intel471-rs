use crate::client::Intel471Client;
use crate::error::Result;
use crate::models::ase::{ListMonitorsRequest, MonitorInfo};
use crate::pagination::build_query;

const SERVICE_PATH: &str = "integrations/ase/v1";

#[derive(Debug, Clone)]
pub struct AseApi<'a> {
    pub(crate) client: &'a Intel471Client,
}

impl<'a> AseApi<'a> {
    pub async fn list_monitors(&self, request: ListMonitorsRequest) -> Result<Vec<MonitorInfo>> {
        let url = self.client.url(SERVICE_PATH);
        let mut params = vec![];
        if let Some(v) = request.last_run_after {
            params.push(("last_run_after", Some(v.to_string())));
        }
        if let Some(v) = request.last_run_before {
            params.push(("last_run_before", Some(v.to_string())));
        }
        let query = build_query(&params);
        let full_url = if query.is_empty() {
            format!("{}/monitor", url)
        } else {
            format!("{}/monitor?{}", url, query)
        };
        self.client.get(&full_url).await
    }
}