use crate::client::Intel471Client;
use crate::error::Result;
use crate::models::tprm::*;
use crate::pagination::build_query;

const SERVICE_PATH: &str = "integrations/tprm/v1";

#[derive(Debug, Clone)]
pub struct TprmApi<'a> {
    pub(crate) client: &'a Intel471Client,
}

impl<'a> TprmApi<'a> {
    pub async fn list_monitors(&self, request: ListMonitorsRequest) -> Result<Vec<MonitorInfo>> {
        let url = self.client.url(SERVICE_PATH);
        let mut params: Vec<(&str, Option<String>)> = vec![];
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
        request: CreateMonitorRequest,
    ) -> Result<CreateMonitorResponse> {
        let url = format!("{}/monitor", self.client.url(SERVICE_PATH));
        self.client.post(&url, &request).await
    }

    pub async fn get_monitor(&self, id: &str) -> Result<MonitorInfo> {
        let url = format!("{}/monitor/{}", self.client.url(SERVICE_PATH), id);
        self.client.get(&url).await
    }

    pub async fn delete_monitor(&self, id: &str) -> Result<()> {
        let url = format!("{}/monitor/{}", self.client.url(SERVICE_PATH), id);
        self.client.delete_void(&url).await
    }

    pub async fn edit_monitor(&self, id: &str, request: EditMonitorRequest) -> Result<()> {
        let url = format!("{}/monitor/{}", self.client.url(SERVICE_PATH), id);
        self.client.patch_void(&url, &request).await
    }

    pub async fn get_monitor_logs(
        &self,
        monitor_id: &str,
        request: GetMonitorLogsRequest,
    ) -> Result<MonitorLogs> {
        let url = format!(
            "{}/monitor/{}/logs",
            self.client.url(SERVICE_PATH),
            monitor_id
        );
        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(v) = request.limit {
            params.push(("limit", Some(v.to_string())));
        }
        if let Some(v) = request.search_text {
            params.push(("search_text", Some(v)));
        }
        if let Some(v) = request.cursor {
            params.push(("cursor", Some(v)));
        }
        if let Some(v) = request.backward {
            params.push(("backward", Some(v.to_string())));
        }
        if let Some(ref v) = request.log_type {
            for item in v {
                params.push(("log_type", Some(item.to_string())));
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

    pub async fn run_monitor(&self, monitor_id: &str, at: Option<u32>) -> Result<()> {
        let url = format!(
            "{}/monitor/{}/run",
            self.client.url(SERVICE_PATH),
            monitor_id
        );
        if let Some(timestamp) = at {
            self.client.post(&url, &timestamp).await
        } else {
            self.client.post(&url, &serde_json::json!(null)).await
        }
    }

    pub async fn get_monitor_scans(
        &self,
        monitor_id: &str,
        status: Option<Vec<ScanStatus>>,
    ) -> Result<Vec<Scan>> {
        let url = format!(
            "{}/monitor/{}/scans",
            self.client.url(SERVICE_PATH),
            monitor_id
        );
        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = status {
            for item in v {
                params.push(("status", Some(item.to_string())));
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
        let mut params: Vec<(&str, Option<String>)> = vec![];
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
        if let Some(v) = request.text {
            params.push(("text", Some(v)));
        }
        if let Some(ref v) = request.sort {
            let val = match v {
                ScanDataSort::Type => "type".to_string(),
                ScanDataSort::module => "module".to_string(),
                ScanDataSort::children => "children".to_string(),
                ScanDataSort::distance => "distance".to_string(),
                ScanDataSort::instances => "instances".to_string(),
                ScanDataSort::data => "data".to_string(),
            };
            params.push(("sort", Some(val)));
        }
        if let Some(v) = request.descending {
            params.push(("descending", Some(v.to_string())));
        }
        if let Some(v) = request.limit {
            params.push(("limit", Some(v.to_string())));
        }
        if let Some(v) = request.cursor {
            params.push(("cursor", Some(v)));
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

    pub async fn get_scan_data_graph(
        &self,
        scan_id: &str,
        category: Option<Vec<String>>,
        type_: Option<Vec<String>>,
        source: Option<Vec<String>>,
        annotated: Option<bool>,
        starred: Option<bool>,
        text: Option<String>,
    ) -> Result<DataGraphResponse> {
        let url = format!(
            "{}/scan/{}/data-graph",
            self.client.url(SERVICE_PATH),
            scan_id
        );
        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = category {
            for item in v {
                params.push(("category", Some(item.clone())));
            }
        }
        if let Some(ref v) = type_ {
            for item in v {
                params.push(("type", Some(item.clone())));
            }
        }
        if let Some(ref v) = source {
            for item in v {
                params.push(("source", Some(item.clone())));
            }
        }
        if let Some(v) = annotated {
            params.push(("annotated", Some(v.to_string())));
        }
        if let Some(v) = starred {
            params.push(("starred", Some(v.to_string())));
        }
        if let Some(v) = text {
            params.push(("text", Some(v)));
        }
        let query = build_query(&params);
        let full_url = if query.is_empty() {
            url
        } else {
            format!("{}?{}", url, query)
        };
        self.client.get(&full_url).await
    }

    pub async fn get_scan_data_tree(
        &self,
        scan_id: &str,
        category: Option<Vec<String>>,
        type_: Option<Vec<String>>,
        source: Option<Vec<String>>,
        annotated: Option<bool>,
        starred: Option<bool>,
        text: Option<String>,
    ) -> Result<TreeNode> {
        let url = format!(
            "{}/scan/{}/data-tree",
            self.client.url(SERVICE_PATH),
            scan_id
        );
        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = category {
            for item in v {
                params.push(("category", Some(item.clone())));
            }
        }
        if let Some(ref v) = type_ {
            for item in v {
                params.push(("type", Some(item.clone())));
            }
        }
        if let Some(ref v) = source {
            for item in v {
                params.push(("source", Some(item.clone())));
            }
        }
        if let Some(v) = annotated {
            params.push(("annotated", Some(v.to_string())));
        }
        if let Some(v) = starred {
            params.push(("starred", Some(v.to_string())));
        }
        if let Some(v) = text {
            params.push(("text", Some(v)));
        }
        let query = build_query(&params);
        let full_url = if query.is_empty() {
            url
        } else {
            format!("{}?{}", url, query)
        };
        self.client.get(&full_url).await
    }

    pub async fn get_scan_data_element(
        &self,
        scan_id: &str,
        element_id: &str,
    ) -> Result<ScanData> {
        let url = format!(
            "{}/scan/{}/data/{}",
            self.client.url(SERVICE_PATH),
            scan_id,
            element_id
        );
        self.client.get(&url).await
    }

    pub async fn patch_scan_data_element(
        &self,
        scan_id: &str,
        element_id: &str,
        request: PatchScanDataElementRequest,
    ) -> Result<()> {
        let url = format!(
            "{}/scan/{}/data/{}",
            self.client.url(SERVICE_PATH),
            scan_id,
            element_id
        );
        self.client.patch_void(&url, &request).await
    }

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

    pub async fn get_scan_data_element_discovery(
        &self,
        scan_id: &str,
        element_id: &str,
    ) -> Result<TreeNode> {
        let url = format!(
            "{}/scan/{}/data/{}/discovery",
            self.client.url(SERVICE_PATH),
            scan_id,
            element_id
        );
        self.client.get(&url).await
    }

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

    pub async fn get_config_current(&self, section: Section) -> Result<ConfigurationOutput> {
        let section_str = match section {
            Section::base => "base",
            Section::passive => "passive",
            Section::essential => "essential",
            Section::full => "full",
        };
        let url = format!(
            "{}/config/current/{}",
            self.client.url(SERVICE_PATH),
            section_str
        );
        self.client.get(&url).await
    }

    pub async fn get_config_defaults(&self, section: Section) -> Result<ConfigurationOutput> {
        let section_str = match section {
            Section::base => "base",
            Section::passive => "passive",
            Section::essential => "essential",
            Section::full => "full",
        };
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
        let section_str = match section {
            Section::base => "base",
            Section::passive => "passive",
            Section::essential => "essential",
            Section::full => "full",
        };
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
        config: &ConfigurationOutput,
        token: &str,
    ) -> Result<()> {
        let section_str = match section {
            Section::base => "base",
            Section::passive => "passive",
            Section::essential => "essential",
            Section::full => "full",
        };
        let url = format!(
            "{}/config/user/{}",
            self.client.url(SERVICE_PATH),
            section_str
        );
        self.client.put_with_bearer_void(&url, token, config).await
    }
}

impl std::fmt::Display for ScanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanStatus::CREATED => write!(f, "CREATED"),
            ScanStatus::INITIALISING => write!(f, "INITIALISING"),
            ScanStatus::STARTED => write!(f, "STARTED"),
            ScanStatus::STARTING => write!(f, "STARTING"),
            ScanStatus::RUNNING => write!(f, "RUNNING"),
            ScanStatus::CORRELATING => write!(f, "CORRELATING"),
            ScanStatus::FINISHED => write!(f, "FINISHED"),
            ScanStatus::ABORTED => write!(f, "ABORTED"),
            ScanStatus::ABORT_REQUESTED => write!(f, "ABORT-REQUESTED"),
            ScanStatus::ERROR_FAILED => write!(f, "ERROR-FAILED"),
            ScanStatus::POST_PROCESSING => write!(f, "POST PROCESSING"),
            ScanStatus::OPEN => write!(f, "OPEN"),
            ScanStatus::ANALYSIS_RUNNING => write!(f, "ANALYSIS RUNNING"),
        }
    }
}

impl std::fmt::Display for LogEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogEntryType::INFO => write!(f, "INFO"),
            LogEntryType::STATUS => write!(f, "STATUS"),
            LogEntryType::ERROR => write!(f, "ERROR"),
            LogEntryType::FATAL => write!(f, "FATAL"),
        }
    }
}