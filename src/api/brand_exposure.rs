use crate::client::Intel471Client;
use crate::error::Result;
use crate::models::brand_exposure::*;
use crate::pagination::build_query;

const SERVICE_PATH: &str = "integrations/brand-exposure/v1";

#[derive(Debug, Clone)]
pub struct BrandExposureApi<'a> {
    pub(crate) client: &'a Intel471Client,
}

impl<'a> BrandExposureApi<'a> {
    pub async fn get_config_current(&self, section: Section) -> Result<ConfigurationOutput> {
        let section_str = serde_json::to_string(&section)
            .unwrap()
            .trim_matches('"')
            .to_string();
        let url = format!(
            "{}/config/current/{}",
            self.client.url(SERVICE_PATH),
            section_str
        );
        self.client.get(&url).await
    }

    pub async fn get_config_defaults(&self, section: Section) -> Result<ConfigurationOutput> {
        let section_str = serde_json::to_string(&section)
            .unwrap()
            .trim_matches('"')
            .to_string();
        let url = format!(
            "{}/config/defaults/{}",
            self.client.url(SERVICE_PATH),
            section_str
        );
        self.client.get(&url).await
    }

    pub async fn get_config_schema(&self) -> Result<OpenApiSchema> {
        let url = format!("{}/config/schema", self.client.url(SERVICE_PATH));
        self.client.get(&url).await
    }

    pub async fn get_config_user(&self, section: Section) -> Result<ConfigurationOutput> {
        let section_str = serde_json::to_string(&section)
            .unwrap()
            .trim_matches('"')
            .to_string();
        let url = format!(
            "{}/config/user/{}",
            self.client.url(SERVICE_PATH),
            section_str
        );
        self.client.get(&url).await
    }

    pub async fn put_config_user(
        &self,
        section: Section,
        config: &ConfigurationInput,
    ) -> Result<()> {
        let section_str = serde_json::to_string(&section)
            .unwrap()
            .trim_matches('"')
            .to_string();
        let url = format!(
            "{}/config/user/{}",
            self.client.url(SERVICE_PATH),
            section_str
        );
        self.client.put_void(&url, config).await
    }

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

    pub async fn create_monitor(
        &self,
        request: &CreateMonitorRequest,
    ) -> Result<CreateMonitorResponse> {
        let url = format!("{}/monitor", self.client.url(SERVICE_PATH));
        self.client.post(&url, request).await
    }

    pub async fn get_monitor(&self, id: &str) -> Result<MonitorInfo> {
        let url = format!("{}/monitor/{}", self.client.url(SERVICE_PATH), id);
        self.client.get(&url).await
    }

    pub async fn delete_monitor(&self, id: &str) -> Result<()> {
        let url = format!("{}/monitor/{}", self.client.url(SERVICE_PATH), id);
        self.client.delete_void(&url).await
    }

    pub async fn edit_monitor(&self, id: &str, request: &EditMonitorRequest) -> Result<()> {
        let url = format!("{}/monitor/{}", self.client.url(SERVICE_PATH), id);
        self.client.patch_void(&url, request).await
    }

    pub async fn get_monitor_logs(
        &self,
        id: &str,
        request: GetMonitorLogsRequest,
    ) -> Result<MonitorLogs> {
        let url = format!(
            "{}/monitor/{}/logs",
            self.client.url(SERVICE_PATH),
            id
        );
        let mut params = vec![];
        if let Some(v) = request.limit {
            params.push(("limit", Some(v.to_string())));
        }
        if let Some(ref v) = request.log_type {
            for item in v {
                let val = serde_json::to_string(item)
                    .unwrap()
                    .trim_matches('"')
                    .to_string();
                params.push(("log_type", Some(val)));
            }
        }
        if let Some(ref v) = request.search_text {
            params.push(("search_text", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }
        if let Some(v) = request.backward {
            params.push(("backward", Some(v.to_string())));
        }
        let query = build_query(&params);
        let full_url = if query.is_empty() {
            url
        } else {
            format!("{}?{}", url, query)
        };
        self.client.get(&full_url).await
    }

    pub async fn run_monitor(&self, id: &str, at: Option<u32>) -> Result<()> {
        let url = format!(
            "{}/monitor/{}/run",
            self.client.url(SERVICE_PATH),
            id
        );
        if let Some(at_value) = at {
            let body = serde_json::json!(at_value);
            self.client.post_void(&url, &body).await
        } else {
            self.client.post_void_empty(&url).await
        }
    }

    pub async fn get_monitor_scans(
        &self,
        id: &str,
        status: Option<&[ScanStatus]>,
    ) -> Result<Vec<Scan>> {
        let url = format!(
            "{}/monitor/{}/scans",
            self.client.url(SERVICE_PATH),
            id
        );
        let mut params = vec![];
        if let Some(v) = status {
            for item in v {
                let val = serde_json::to_string(item)
                    .unwrap()
                    .trim_matches('"')
                    .to_string();
                params.push(("status", Some(val)));
            }
        }
        let query = build_query(&params);
        let full_url = if query.is_empty() {
            url
        } else {
            format!("{}?{}", url, query)
        };
        self.client.get(&full_url).await
    }

    #[deprecated(note = "Deprecated in the Brand Exposure API")]
    pub async fn get_scan_data(
        &self,
        scan_id: &str,
        request: GetScanDataRequest,
    ) -> Result<ScanDataResponse> {
        let url = format!(
            "{}/scan/{}/data",
            self.client.url(SERVICE_PATH),
            scan_id
        );
        let mut params = vec![];
        if let Some(ref v) = request.category {
            for item in v {
                params.push(("category", Some(item.clone())));
            }
        }
        if let Some(ref v) = request.type_ {
            for item in v {
                params.push(("type", Some(item.clone())));
            }
        }
        if let Some(ref v) = request.source {
            for item in v {
                params.push(("source", Some(item.clone())));
            }
        }
        if let Some(v) = request.annotated {
            params.push(("annotated", Some(v.to_string())));
        }
        if let Some(v) = request.starred {
            params.push(("starred", Some(v.to_string())));
        }
        if let Some(ref v) = request.text {
            params.push(("text", Some(v.clone())));
        }
        if let Some(ref v) = request.sort {
            let val = serde_json::to_string(v)
                .unwrap()
                .trim_matches('"')
                .to_string();
            params.push(("sort", Some(val)));
        }
        if let Some(v) = request.descending {
            params.push(("descending", Some(v.to_string())));
        }
        if let Some(v) = request.limit {
            params.push(("limit", Some(v.to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }
        if let Some(v) = request.backward {
            params.push(("backward", Some(v.to_string())));
        }
        let query = build_query(&params);
        let full_url = if query.is_empty() {
            url
        } else {
            format!("{}?{}", url, query)
        };
        self.client.get(&full_url).await
    }

    #[deprecated(note = "Deprecated in the Brand Exposure API")]
    pub async fn get_scan_data_graph(
        &self,
        scan_id: &str,
        request: GetScanDataGraphRequest,
    ) -> Result<DataGraphResponse> {
        let url = format!(
            "{}/scan/{}/data-graph",
            self.client.url(SERVICE_PATH),
            scan_id
        );
        let mut params = vec![];
        if let Some(ref v) = request.category {
            for item in v {
                params.push(("category", Some(item.clone())));
            }
        }
        if let Some(ref v) = request.type_ {
            for item in v {
                params.push(("type", Some(item.clone())));
            }
        }
        if let Some(ref v) = request.source {
            for item in v {
                params.push(("source", Some(item.clone())));
            }
        }
        if let Some(v) = request.annotated {
            params.push(("annotated", Some(v.to_string())));
        }
        if let Some(v) = request.starred {
            params.push(("starred", Some(v.to_string())));
        }
        if let Some(ref v) = request.text {
            params.push(("text", Some(v.clone())));
        }
        let query = build_query(&params);
        let full_url = if query.is_empty() {
            url
        } else {
            format!("{}?{}", url, query)
        };
        self.client.get(&full_url).await
    }

    #[deprecated(note = "Deprecated in the Brand Exposure API")]
    pub async fn get_scan_data_tree(
        &self,
        scan_id: &str,
        request: GetScanDataTreeRequest,
    ) -> Result<ScanApiTreeNode> {
        let url = format!(
            "{}/scan/{}/data-tree",
            self.client.url(SERVICE_PATH),
            scan_id
        );
        let mut params = vec![];
        if let Some(ref v) = request.category {
            for item in v {
                params.push(("category", Some(item.clone())));
            }
        }
        if let Some(ref v) = request.type_ {
            for item in v {
                params.push(("type", Some(item.clone())));
            }
        }
        if let Some(ref v) = request.source {
            for item in v {
                params.push(("source", Some(item.clone())));
            }
        }
        if let Some(v) = request.annotated {
            params.push(("annotated", Some(v.to_string())));
        }
        if let Some(v) = request.starred {
            params.push(("starred", Some(v.to_string())));
        }
        if let Some(ref v) = request.text {
            params.push(("text", Some(v.clone())));
        }
        let query = build_query(&params);
        let full_url = if query.is_empty() {
            url
        } else {
            format!("{}?{}", url, query)
        };
        self.client.get(&full_url).await
    }

    #[deprecated(note = "Deprecated in the Brand Exposure API")]
    pub async fn get_scan_data_element(
        &self,
        scan_id: &str,
        element_id: &str,
    ) -> Result<DataElement> {
        let url = format!(
            "{}/scan/{}/data/{}",
            self.client.url(SERVICE_PATH),
            scan_id,
            element_id
        );
        self.client.get(&url).await
    }

    #[deprecated(note = "Deprecated in the Brand Exposure API")]
    pub async fn patch_scan_data_element(
        &self,
        scan_id: &str,
        element_id: &str,
        request: &PatchScanDataElementRequest,
    ) -> Result<()> {
        let url = format!(
            "{}/scan/{}/data/{}",
            self.client.url(SERVICE_PATH),
            scan_id,
            element_id
        );
        self.client.patch_void(&url, request).await
    }

    #[deprecated(note = "Deprecated in the Brand Exposure API")]
    pub async fn get_scan_data_element_children(
        &self,
        scan_id: &str,
        element_id: &str,
    ) -> Result<Vec<DataEvent>> {
        let url = format!(
            "{}/scan/{}/data/{}/children",
            self.client.url(SERVICE_PATH),
            scan_id,
            element_id
        );
        self.client.get(&url).await
    }

    #[deprecated(note = "Deprecated in the Brand Exposure API")]
    pub async fn get_scan_data_element_discovery(
        &self,
        scan_id: &str,
        element_id: &str,
    ) -> Result<ScanApiTreeNode> {
        let url = format!(
            "{}/scan/{}/data/{}/discovery",
            self.client.url(SERVICE_PATH),
            scan_id,
            element_id
        );
        self.client.get(&url).await
    }

    #[deprecated(note = "Deprecated in the Brand Exposure API")]
    pub async fn get_scan_data_element_events(
        &self,
        scan_id: &str,
        element_id: &str,
    ) -> Result<Vec<DataEvent>> {
        let url = format!(
            "{}/scan/{}/data/{}/events",
            self.client.url(SERVICE_PATH),
            scan_id,
            element_id
        );
        self.client.get(&url).await
    }

    #[deprecated(note = "Deprecated in the Brand Exposure API")]
    pub async fn get_scan_data_element_relationships(
        &self,
        scan_id: &str,
        element_id: &str,
    ) -> Result<DataGraph> {
        let url = format!(
            "{}/scan/{}/data/{}/relationships",
            self.client.url(SERVICE_PATH),
            scan_id,
            element_id
        );
        self.client.get(&url).await
    }
}