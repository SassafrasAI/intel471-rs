use crate::client::Intel471Client;
use crate::error::Result;
use crate::models::reports::*;
use crate::pagination::{PaginatedStream, build_query_with_int};

const SERVICE_PATH: &str = "integrations/intel-report/v1";

#[derive(Debug, Clone)]
pub struct ReportsApi<'a> {
    pub(crate) client: &'a Intel471Client,
}

impl<'a> ReportsApi<'a> {
    pub fn stream(&self, request: ReportsStreamRequest) -> Result<PaginatedStream<'a, ReportResponseStream>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/reports/stream", url);

        let mut str_params = vec![];
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }
        if let Some(ref types) = request.type_ {
            let joined = types.iter().map(|t| {
                let s = serde_json::to_value(t).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("type", Some(joined)));
        }
        if let Some(ref sub_types) = request.sub_type {
            let joined = sub_types.join(",");
            str_params.push(("sub_type", Some(joined)));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &ReportResponseStream| page.cursor_next.clone(),
        ))
    }

    pub async fn stream_page(&self, request: ReportsStreamRequest) -> Result<ReportResponseStream> {
        let url = self.client.url(SERVICE_PATH);
        let mut str_params = vec![];
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }
        if let Some(ref types) = request.type_ {
            let joined = types.iter().map(|t| {
                let s = serde_json::to_value(t).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("type", Some(joined)));
        }
        if let Some(ref sub_types) = request.sub_type {
            let joined = sub_types.join(",");
            str_params.push(("sub_type", Some(joined)));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);
        let full_url = if query.is_empty() {
            format!("{}/reports/stream", url)
        } else {
            format!("{}/reports/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub fn fintel_stream(&self, request: FintelStreamRequest) -> Result<PaginatedStream<'a, FintelReportsResponseStream>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/reports/fintel/stream", url);

        let mut str_params = vec![];
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }
        if let Some(ref sub_types) = request.sub_type {
            let joined = sub_types.iter().map(|t| {
                let s = serde_json::to_value(t).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("sub_type", Some(joined)));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &FintelReportsResponseStream| page.cursor_next.clone(),
        ))
    }

    pub async fn fintel_stream_page(&self, request: FintelStreamRequest) -> Result<FintelReportsResponseStream> {
        let url = self.client.url(SERVICE_PATH);
        let mut str_params = vec![];
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }
        if let Some(ref sub_types) = request.sub_type {
            let joined = sub_types.iter().map(|t| {
                let s = serde_json::to_value(t).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("sub_type", Some(joined)));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);
        let full_url = if query.is_empty() {
            format!("{}/reports/fintel/stream", url)
        } else {
            format!("{}/reports/fintel/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn fintel_detail(&self, id: &str, include_inline_images: Option<bool>) -> Result<FintelResponse> {
        let url = self.client.url(SERVICE_PATH);
        let base = format!("{}/reports/fintel/{}", url, id);
        let full_url = match include_inline_images {
            Some(true) => format!("{}?include_inline_images=true", base),
            _ => base,
        };
        self.client.get(&full_url).await
    }

    pub async fn fintel_attachment(&self, report_id: &str, attachment_id: &str) -> Result<reqwest::Response> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/reports/fintel/{}/attachments/{}", url, report_id, attachment_id);
        self.client.get_raw(&full_url).await
    }

    pub fn breach_alert_stream(&self, request: BreachAlertStreamRequest) -> Result<PaginatedStream<'a, BreachAlertsResponseStream>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/reports/breach-alert/stream", url);

        let mut str_params = vec![];
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &BreachAlertsResponseStream| page.cursor_next.clone(),
        ))
    }

    pub async fn breach_alert_stream_page(&self, request: BreachAlertStreamRequest) -> Result<BreachAlertsResponseStream> {
        let url = self.client.url(SERVICE_PATH);
        let mut str_params = vec![];
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);
        let full_url = if query.is_empty() {
            format!("{}/reports/breach-alert/stream", url)
        } else {
            format!("{}/reports/breach-alert/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn breach_alert_detail(&self, id: &str, include_inline_images: Option<bool>) -> Result<BreachAlertResponse> {
        let url = self.client.url(SERVICE_PATH);
        let base = format!("{}/reports/breach-alert/{}", url, id);
        let full_url = match include_inline_images {
            Some(true) => format!("{}?include_inline_images=true", base),
            _ => base,
        };
        self.client.get(&full_url).await
    }

    pub fn geopol_stream(&self, request: GeopolStreamRequest) -> Result<PaginatedStream<'a, GeopolReportsResponseStream>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/reports/geopol/stream", url);

        let mut str_params = vec![];
        if let Some(ref v) = request.country {
            str_params.push(("country", Some(v.clone())));
        }
        if let Some(ref v) = request.report_location_country {
            str_params.push(("report_location_country", Some(v.clone())));
        }
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }
        if let Some(ref sub_types) = request.sub_type {
            let joined = sub_types.iter().map(|t| {
                let s = serde_json::to_value(t).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("sub_type", Some(joined)));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &GeopolReportsResponseStream| page.cursor_next.clone(),
        ))
    }

    pub async fn geopol_stream_page(&self, request: GeopolStreamRequest) -> Result<GeopolReportsResponseStream> {
        let url = self.client.url(SERVICE_PATH);
        let mut str_params = vec![];
        if let Some(ref v) = request.country {
            str_params.push(("country", Some(v.clone())));
        }
        if let Some(ref v) = request.report_location_country {
            str_params.push(("report_location_country", Some(v.clone())));
        }
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }
        if let Some(ref sub_types) = request.sub_type {
            let joined = sub_types.iter().map(|t| {
                let s = serde_json::to_value(t).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("sub_type", Some(joined)));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);
        let full_url = if query.is_empty() {
            format!("{}/reports/geopol/stream", url)
        } else {
            format!("{}/reports/geopol/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn geopol_detail(&self, id: &str, include_inline_images: Option<bool>) -> Result<GeopolReportDetailsResponse> {
        let url = self.client.url(SERVICE_PATH);
        let base = format!("{}/reports/geopol/{}", url, id);
        let full_url = match include_inline_images {
            Some(true) => format!("{}?include_inline_images=true", base),
            _ => base,
        };
        self.client.get(&full_url).await
    }

    pub async fn geopol_attachment(&self, report_id: &str, attachment_id: &str) -> Result<reqwest::Response> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/reports/geopol/{}/attachments/{}", url, report_id, attachment_id);
        self.client.get_raw(&full_url).await
    }

    pub fn info_stream(&self, request: InfoStreamRequest) -> Result<PaginatedStream<'a, InfoReportsResponseStream>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/reports/info/stream", url);

        let mut str_params = vec![];
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &InfoReportsResponseStream| page.cursor_next.clone(),
        ))
    }

    pub async fn info_stream_page(&self, request: InfoStreamRequest) -> Result<InfoReportsResponseStream> {
        let url = self.client.url(SERVICE_PATH);
        let mut str_params = vec![];
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);
        let full_url = if query.is_empty() {
            format!("{}/reports/info/stream", url)
        } else {
            format!("{}/reports/info/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn info_detail(&self, id: &str, include_inline_images: Option<bool>) -> Result<InfoReportResponse> {
        let url = self.client.url(SERVICE_PATH);
        let base = format!("{}/reports/info/{}", url, id);
        let full_url = match include_inline_images {
            Some(true) => format!("{}?include_inline_images=true", base),
            _ => base,
        };
        self.client.get(&full_url).await
    }

    pub async fn info_attachment(&self, report_id: &str, attachment_id: &str) -> Result<reqwest::Response> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/reports/info/{}/attachments/{}", url, report_id, attachment_id);
        self.client.get_raw(&full_url).await
    }

    pub fn malware_stream(&self, request: MalwareStreamRequest) -> Result<PaginatedStream<'a, MalwareReportsResponseStream>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/reports/malware/stream", url);

        let mut str_params = vec![];
        if let Some(ref v) = request.malware_family {
            str_params.push(("malware_family", Some(v.clone())));
        }
        if let Some(ref v) = request.threat_id {
            str_params.push(("threat_id", Some(v.clone())));
        }
        if let Some(ref v) = request.threat_type {
            str_params.push(("threat_type", Some(v.clone())));
        }
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &MalwareReportsResponseStream| page.cursor_next.clone(),
        ))
    }

    pub async fn malware_stream_page(&self, request: MalwareStreamRequest) -> Result<MalwareReportsResponseStream> {
        let url = self.client.url(SERVICE_PATH);
        let mut str_params = vec![];
        if let Some(ref v) = request.malware_family {
            str_params.push(("malware_family", Some(v.clone())));
        }
        if let Some(ref v) = request.threat_id {
            str_params.push(("threat_id", Some(v.clone())));
        }
        if let Some(ref v) = request.threat_type {
            str_params.push(("threat_type", Some(v.clone())));
        }
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);
        let full_url = if query.is_empty() {
            format!("{}/reports/malware/stream", url)
        } else {
            format!("{}/reports/malware/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn malware_detail(&self, id: &str, include_inline_images: Option<bool>) -> Result<MalwareReportResponse> {
        let url = self.client.url(SERVICE_PATH);
        let base = format!("{}/reports/malware/{}", url, id);
        let full_url = match include_inline_images {
            Some(true) => format!("{}?include_inline_images=true", base),
            _ => base,
        };
        self.client.get(&full_url).await
    }

    pub async fn malware_attachment(&self, report_id: &str, attachment_id: &str) -> Result<reqwest::Response> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/reports/malware/{}/attachments/{}", url, report_id, attachment_id);
        self.client.get_raw(&full_url).await
    }

    pub fn spot_stream(&self, request: SpotStreamRequest) -> Result<PaginatedStream<'a, SpotReportsResponseStream>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/reports/spot/stream", url);

        let mut str_params = vec![];
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &SpotReportsResponseStream| page.cursor_next.clone(),
        ))
    }

    pub async fn spot_stream_page(&self, request: SpotStreamRequest) -> Result<SpotReportsResponseStream> {
        let url = self.client.url(SERVICE_PATH);
        let mut str_params = vec![];
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);
        let full_url = if query.is_empty() {
            format!("{}/reports/spot/stream", url)
        } else {
            format!("{}/reports/spot/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn spot_detail(&self, id: &str, include_inline_images: Option<bool>) -> Result<SpotReportResponse> {
        let url = self.client.url(SERVICE_PATH);
        let base = format!("{}/reports/spot/{}", url, id);
        let full_url = match include_inline_images {
            Some(true) => format!("{}?include_inline_images=true", base),
            _ => base,
        };
        self.client.get(&full_url).await
    }

    pub fn vulnerability_stream(&self, request: VulnerabilityStreamRequest) -> Result<PaginatedStream<'a, VulnerabilitiesReportsResponseStream>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/reports/vulnerability/stream", url);

        let mut str_params = vec![];
        if let Some(ref v) = request.status {
            let s = serde_json::to_value(v).unwrap();
            str_params.push(("status", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cve_type {
            str_params.push(("cve_type", Some(v.clone())));
        }
        if let Some(ref v) = request.cve_name {
            str_params.push(("cve_name", Some(v.clone())));
        }
        if let Some(ref v) = request.vendor_name {
            str_params.push(("vendor_name", Some(v.clone())));
        }
        if let Some(ref v) = request.product_name {
            str_params.push(("product_name", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }
        if let Some(ref vals) = request.risk_level {
            let joined = vals.iter().map(|v| {
                let s = serde_json::to_value(v).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("risk_level", Some(joined)));
        }
        if let Some(ref vals) = request.patch_status {
            let joined = vals.iter().map(|v| {
                let s = serde_json::to_value(v).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("patch_status", Some(joined)));
        }
        if let Some(ref vals) = request.interest_level {
            let joined = vals.iter().map(|v| {
                let s = serde_json::to_value(v).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("interest_level", Some(joined)));
        }
        if let Some(ref vals) = request.activity_location {
            let joined = vals.iter().map(|v| {
                let s = serde_json::to_value(v).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("activity_location", Some(joined)));
        }
        if let Some(ref vals) = request.exploit_status {
            let joined = vals.iter().map(|v| {
                let s = serde_json::to_value(v).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("exploit_status", Some(joined)));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            query,
            |page: &VulnerabilitiesReportsResponseStream| page.cursor_next.clone(),
        ))
    }

    pub async fn vulnerability_stream_page(&self, request: VulnerabilityStreamRequest) -> Result<VulnerabilitiesReportsResponseStream> {
        let url = self.client.url(SERVICE_PATH);
        let mut str_params = vec![];
        if let Some(ref v) = request.status {
            let s = serde_json::to_value(v).unwrap();
            str_params.push(("status", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.text_filter {
            str_params.push(("text_filter", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            str_params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.cve_type {
            str_params.push(("cve_type", Some(v.clone())));
        }
        if let Some(ref v) = request.cve_name {
            str_params.push(("cve_name", Some(v.clone())));
        }
        if let Some(ref v) = request.vendor_name {
            str_params.push(("vendor_name", Some(v.clone())));
        }
        if let Some(ref v) = request.product_name {
            str_params.push(("product_name", Some(v.clone())));
        }
        if let Some(ref v) = request.cursor {
            str_params.push(("cursor", Some(v.clone())));
        }
        if let Some(ref vals) = request.risk_level {
            let joined = vals.iter().map(|v| {
                let s = serde_json::to_value(v).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("risk_level", Some(joined)));
        }
        if let Some(ref vals) = request.patch_status {
            let joined = vals.iter().map(|v| {
                let s = serde_json::to_value(v).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("patch_status", Some(joined)));
        }
        if let Some(ref vals) = request.interest_level {
            let joined = vals.iter().map(|v| {
                let s = serde_json::to_value(v).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("interest_level", Some(joined)));
        }
        if let Some(ref vals) = request.activity_location {
            let joined = vals.iter().map(|v| {
                let s = serde_json::to_value(v).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("activity_location", Some(joined)));
        }
        if let Some(ref vals) = request.exploit_status {
            let joined = vals.iter().map(|v| {
                let s = serde_json::to_value(v).unwrap();
                s.as_str().unwrap().to_string()
            }).collect::<Vec<_>>().join(",");
            str_params.push(("exploit_status", Some(joined)));
        }

        let int_params = vec![
            ("from", request.from),
            ("until", request.until),
            ("size", request.size.map(|s| s as i64)),
        ];

        let query = build_query_with_int(&str_params, &int_params);
        let full_url = if query.is_empty() {
            format!("{}/reports/vulnerability/stream", url)
        } else {
            format!("{}/reports/vulnerability/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }

    pub async fn vulnerability_detail(&self, id: &str) -> Result<VulnerabilitiesReportDetailsResponse> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/reports/vulnerability/{}", url, id);
        self.client.get(&full_url).await
    }

    pub async fn vulnerability_download_pdf(&self, id: &str) -> Result<reqwest::Response> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/reports/vulnerability/{}/download-as-pdf", url, id);
        self.client.get_raw(&full_url).await
    }

    pub async fn download_pdf(&self, id: &str) -> Result<reqwest::Response> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/reports/{}/download-as-pdf", url, id);
        self.client.get_raw(&full_url).await
    }
}