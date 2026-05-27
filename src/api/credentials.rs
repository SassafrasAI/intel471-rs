use crate::client::Intel471Client;
use crate::error::Result;
use crate::models::credentials::{
    CredentialOccurrence, CredentialOccurrenceStreamPage, CredentialOccurrenceStreamRequest,
    CredentialSet, CredentialSetAccessedUrlStreamPage,
    CredentialSetAccessedUrlStreamRequest, CredentialSetStreamPage, CredentialSetStreamRequest,
    CredentialStreamPage, CredentialStreamRequest, Credential,
};
use crate::pagination::{PaginatedStream, build_query};

const SERVICE_PATH: &str = "integrations/creds/v1";

#[derive(Debug, Clone)]
pub struct CredentialsApi<'a> {
    pub(crate) client: &'a Intel471Client,
}

impl<'a> CredentialsApi<'a> {
    pub fn credential_stream(
        &self,
        request: CredentialStreamRequest,
    ) -> Result<PaginatedStream<'a, CredentialStreamPage>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/credentials/stream", url);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.credential_set_name {
            params.push(("credential_set_name", Some(v.clone())));
        }
        if let Some(ref v) = request.credential_set_id {
            params.push(("credential_set_id", Some(v.clone())));
        }
        if let Some(ref v) = request.domain {
            params.push(("domain", Some(v.clone())));
        }
        if let Some(ref v) = request.affiliation_group {
            let s = serde_json::to_value(v).unwrap();
            params.push(("affiliation_group", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.password_strength {
            let s = serde_json::to_value(v).unwrap();
            params.push(("password_strength", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(v) = request.password_length_gte {
            params.push(("password_length_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_lowercase_gte {
            params.push(("password_lowercase_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_uppercase_gte {
            params.push(("password_uppercase_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_numbers_gte {
            params.push(("password_numbers_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_punctuation_gte {
            params.push(("password_punctuation_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_symbols_gte {
            params.push(("password_symbols_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_separators_gte {
            params.push(("password_separators_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_other_gte {
            params.push(("password_other_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_entropy_gte {
            params.push(("password_entropy_gte", Some(v.to_string())));
        }
        if let Some(ref v) = request.password_plain {
            params.push(("password_plain", Some(v.clone())));
        }
        if let Some(ref v) = request.credential_login {
            params.push(("credential_login", Some(v.clone())));
        }
        if let Some(ref v) = request.detected_malware {
            params.push(("detected_malware", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            params.push(("girs", Some(v.clone())));
        }
        if let Some(v) = request.from {
            params.push(("from", Some(v.to_string())));
        }
        if let Some(v) = request.until {
            params.push(("until", Some(v.to_string())));
        }
        if let Some(ref v) = request.last_updated_from {
            params.push(("last_updated_from", Some(v.clone())));
        }
        if let Some(ref v) = request.last_updated_until {
            params.push(("last_updated_until", Some(v.clone())));
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
            |page: &CredentialStreamPage| page.cursor_next.clone(),
        ))
    }

    pub async fn credential_stream_page(
        &self,
        request: CredentialStreamRequest,
    ) -> Result<CredentialStreamPage> {
        let url = self.client.url(SERVICE_PATH);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.credential_set_name {
            params.push(("credential_set_name", Some(v.clone())));
        }
        if let Some(ref v) = request.credential_set_id {
            params.push(("credential_set_id", Some(v.clone())));
        }
        if let Some(ref v) = request.domain {
            params.push(("domain", Some(v.clone())));
        }
        if let Some(ref v) = request.affiliation_group {
            let s = serde_json::to_value(v).unwrap();
            params.push(("affiliation_group", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.password_strength {
            let s = serde_json::to_value(v).unwrap();
            params.push(("password_strength", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(v) = request.password_length_gte {
            params.push(("password_length_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_lowercase_gte {
            params.push(("password_lowercase_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_uppercase_gte {
            params.push(("password_uppercase_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_numbers_gte {
            params.push(("password_numbers_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_punctuation_gte {
            params.push(("password_punctuation_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_symbols_gte {
            params.push(("password_symbols_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_separators_gte {
            params.push(("password_separators_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_other_gte {
            params.push(("password_other_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_entropy_gte {
            params.push(("password_entropy_gte", Some(v.to_string())));
        }
        if let Some(ref v) = request.password_plain {
            params.push(("password_plain", Some(v.clone())));
        }
        if let Some(ref v) = request.credential_login {
            params.push(("credential_login", Some(v.clone())));
        }
        if let Some(ref v) = request.detected_malware {
            params.push(("detected_malware", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            params.push(("girs", Some(v.clone())));
        }
        if let Some(v) = request.from {
            params.push(("from", Some(v.to_string())));
        }
        if let Some(v) = request.until {
            params.push(("until", Some(v.to_string())));
        }
        if let Some(ref v) = request.last_updated_from {
            params.push(("last_updated_from", Some(v.clone())));
        }
        if let Some(ref v) = request.last_updated_until {
            params.push(("last_updated_until", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }

        let query = build_query(&params);
        let full_url = format!("{}/credentials/stream?{}", url, query);
        self.client.get(&full_url).await
    }

    pub fn credential_occurrence_stream(
        &self,
        request: CredentialOccurrenceStreamRequest,
    ) -> Result<PaginatedStream<'a, CredentialOccurrenceStreamPage>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/credentials/occurrences/stream", url);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.credential_id {
            params.push(("credential_id", Some(v.clone())));
        }
        if let Some(ref v) = request.credential_set_name {
            params.push(("credential_set_name", Some(v.clone())));
        }
        if let Some(ref v) = request.credential_set_id {
            params.push(("credential_set_id", Some(v.clone())));
        }
        if let Some(ref v) = request.domain {
            params.push(("domain", Some(v.clone())));
        }
        if let Some(ref v) = request.affiliation_group {
            let s = serde_json::to_value(v).unwrap();
            params.push(("affiliation_group", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.password_strength {
            let s = serde_json::to_value(v).unwrap();
            params.push(("password_strength", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(v) = request.password_length_gte {
            params.push(("password_length_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_lowercase_gte {
            params.push(("password_lowercase_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_uppercase_gte {
            params.push(("password_uppercase_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_numbers_gte {
            params.push(("password_numbers_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_punctuation_gte {
            params.push(("password_punctuation_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_symbols_gte {
            params.push(("password_symbols_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_separators_gte {
            params.push(("password_separators_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_other_gte {
            params.push(("password_other_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_entropy_gte {
            params.push(("password_entropy_gte", Some(v.to_string())));
        }
        if let Some(ref v) = request.password_plain {
            params.push(("password_plain", Some(v.clone())));
        }
        if let Some(ref v) = request.credential_login {
            params.push(("credential_login", Some(v.clone())));
        }
        if let Some(ref v) = request.detected_malware {
            params.push(("detected_malware", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            params.push(("girs", Some(v.clone())));
        }
        if let Some(v) = request.from {
            params.push(("from", Some(v.to_string())));
        }
        if let Some(v) = request.until {
            params.push(("until", Some(v.to_string())));
        }
        if let Some(ref v) = request.last_updated_from {
            params.push(("last_updated_from", Some(v.clone())));
        }
        if let Some(ref v) = request.last_updated_until {
            params.push(("last_updated_until", Some(v.clone())));
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
            |page: &CredentialOccurrenceStreamPage| page.cursor_next.clone(),
        ))
    }

    pub async fn credential_occurrence_stream_page(
        &self,
        request: CredentialOccurrenceStreamRequest,
    ) -> Result<CredentialOccurrenceStreamPage> {
        let url = self.client.url(SERVICE_PATH);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.credential_id {
            params.push(("credential_id", Some(v.clone())));
        }
        if let Some(ref v) = request.credential_set_name {
            params.push(("credential_set_name", Some(v.clone())));
        }
        if let Some(ref v) = request.credential_set_id {
            params.push(("credential_set_id", Some(v.clone())));
        }
        if let Some(ref v) = request.domain {
            params.push(("domain", Some(v.clone())));
        }
        if let Some(ref v) = request.affiliation_group {
            let s = serde_json::to_value(v).unwrap();
            params.push(("affiliation_group", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(ref v) = request.password_strength {
            let s = serde_json::to_value(v).unwrap();
            params.push(("password_strength", Some(s.as_str().unwrap().to_string())));
        }
        if let Some(v) = request.password_length_gte {
            params.push(("password_length_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_lowercase_gte {
            params.push(("password_lowercase_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_uppercase_gte {
            params.push(("password_uppercase_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_numbers_gte {
            params.push(("password_numbers_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_punctuation_gte {
            params.push(("password_punctuation_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_symbols_gte {
            params.push(("password_symbols_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_separators_gte {
            params.push(("password_separators_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_other_gte {
            params.push(("password_other_gte", Some(v.to_string())));
        }
        if let Some(v) = request.password_entropy_gte {
            params.push(("password_entropy_gte", Some(v.to_string())));
        }
        if let Some(ref v) = request.password_plain {
            params.push(("password_plain", Some(v.clone())));
        }
        if let Some(ref v) = request.credential_login {
            params.push(("credential_login", Some(v.clone())));
        }
        if let Some(ref v) = request.detected_malware {
            params.push(("detected_malware", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            params.push(("girs", Some(v.clone())));
        }
        if let Some(v) = request.from {
            params.push(("from", Some(v.to_string())));
        }
        if let Some(v) = request.until {
            params.push(("until", Some(v.to_string())));
        }
        if let Some(ref v) = request.last_updated_from {
            params.push(("last_updated_from", Some(v.clone())));
        }
        if let Some(ref v) = request.last_updated_until {
            params.push(("last_updated_until", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }

        let query = build_query(&params);
        let full_url = format!("{}/credentials/occurrences/stream?{}", url, query);
        self.client.get(&full_url).await
    }

    pub fn credential_set_stream(
        &self,
        request: CredentialSetStreamRequest,
    ) -> Result<PaginatedStream<'a, CredentialSetStreamPage>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/credential-sets/stream", url);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.credential_set_name {
            params.push(("credential_set_name", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.victim {
            params.push(("victim", Some(v.clone())));
        }
        if let Some(v) = request.from {
            params.push(("from", Some(v.to_string())));
        }
        if let Some(v) = request.until {
            params.push(("until", Some(v.to_string())));
        }
        if let Some(ref v) = request.last_updated_from {
            params.push(("last_updated_from", Some(v.clone())));
        }
        if let Some(ref v) = request.last_updated_until {
            params.push(("last_updated_until", Some(v.clone())));
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
            |page: &CredentialSetStreamPage| page.cursor_next.clone(),
        ))
    }

    pub async fn credential_set_stream_page(
        &self,
        request: CredentialSetStreamRequest,
    ) -> Result<CredentialSetStreamPage> {
        let url = self.client.url(SERVICE_PATH);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.credential_set_name {
            params.push(("credential_set_name", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.victim {
            params.push(("victim", Some(v.clone())));
        }
        if let Some(v) = request.from {
            params.push(("from", Some(v.to_string())));
        }
        if let Some(v) = request.until {
            params.push(("until", Some(v.to_string())));
        }
        if let Some(ref v) = request.last_updated_from {
            params.push(("last_updated_from", Some(v.clone())));
        }
        if let Some(ref v) = request.last_updated_until {
            params.push(("last_updated_until", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }

        let query = build_query(&params);
        let full_url = format!("{}/credential-sets/stream?{}", url, query);
        self.client.get(&full_url).await
    }

    pub fn credential_set_accessed_url_stream(
        &self,
        request: CredentialSetAccessedUrlStreamRequest,
    ) -> Result<PaginatedStream<'a, CredentialSetAccessedUrlStreamPage>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/credential-sets/accessed-urls/stream", url);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.credential_set_id {
            params.push(("credential_set_id", Some(v.clone())));
        }
        if let Some(ref v) = request.credential_set_name {
            params.push(("credential_set_name", Some(v.clone())));
        }
        if let Some(ref v) = request.accessed_url {
            params.push(("accessed_url", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.victim {
            params.push(("victim", Some(v.clone())));
        }
        if let Some(v) = request.from {
            params.push(("from", Some(v.to_string())));
        }
        if let Some(v) = request.until {
            params.push(("until", Some(v.to_string())));
        }
        if let Some(ref v) = request.last_updated_from {
            params.push(("last_updated_from", Some(v.clone())));
        }
        if let Some(ref v) = request.last_updated_until {
            params.push(("last_updated_until", Some(v.clone())));
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
            |page: &CredentialSetAccessedUrlStreamPage| page.cursor_next.clone(),
        ))
    }

    pub async fn credential_set_accessed_url_stream_page(
        &self,
        request: CredentialSetAccessedUrlStreamRequest,
    ) -> Result<CredentialSetAccessedUrlStreamPage> {
        let url = self.client.url(SERVICE_PATH);

        let mut params: Vec<(&str, Option<String>)> = vec![];
        if let Some(ref v) = request.credential_set_id {
            params.push(("credential_set_id", Some(v.clone())));
        }
        if let Some(ref v) = request.credential_set_name {
            params.push(("credential_set_name", Some(v.clone())));
        }
        if let Some(ref v) = request.accessed_url {
            params.push(("accessed_url", Some(v.clone())));
        }
        if let Some(ref v) = request.girs {
            params.push(("girs", Some(v.clone())));
        }
        if let Some(ref v) = request.victim {
            params.push(("victim", Some(v.clone())));
        }
        if let Some(v) = request.from {
            params.push(("from", Some(v.to_string())));
        }
        if let Some(v) = request.until {
            params.push(("until", Some(v.to_string())));
        }
        if let Some(ref v) = request.last_updated_from {
            params.push(("last_updated_from", Some(v.clone())));
        }
        if let Some(ref v) = request.last_updated_until {
            params.push(("last_updated_until", Some(v.clone())));
        }
        if let Some(v) = request.size {
            params.push(("size", Some(v.to_string())));
        }
        if let Some(ref v) = request.cursor {
            params.push(("cursor", Some(v.clone())));
        }

        let query = build_query(&params);
        let full_url = format!("{}/credential-sets/accessed-urls/stream?{}", url, query);
        self.client.get(&full_url).await
    }

    pub async fn get_credential(&self, id: &str) -> Result<Credential> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/credentials/{}", url, id);
        self.client.get(&full_url).await
    }

    pub async fn get_credential_occurrence(&self, id: &str) -> Result<CredentialOccurrence> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/credentials/occurrences/{}", url, id);
        self.client.get(&full_url).await
    }

    pub async fn get_credential_set(&self, id: &str) -> Result<CredentialSet> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/credential-sets/{}", url, id);
        self.client.get(&full_url).await
    }
}