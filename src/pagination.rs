use crate::error::Result;
use crate::Intel471Client;
use std::sync::Arc;

type CursorExtractor<T> = fn(&T) -> Option<String>;

pub struct PaginatedStream<'a, T> {
    client: &'a Intel471Client,
    base_url: String,
    params: String,
    cursor: Option<String>,
    exhausted: bool,
    extract_cursor: CursorExtractor<T>,
}

impl<'a, T> PaginatedStream<'a, T> {
    pub fn new(
        client: &'a Intel471Client,
        base_url: String,
        params: String,
        extract_cursor: CursorExtractor<T>,
    ) -> Self {
        Self {
            client,
            base_url,
            params,
            cursor: None,
            exhausted: false,
            extract_cursor,
        }
    }
}

impl<'a, T: serde::de::DeserializeOwned + Send + 'a> PaginatedStream<'a, T> {
    pub async fn next_page(&mut self) -> Result<Option<T>> {
        if self.exhausted {
            return Ok(None);
        }

        let mut url = if let Some(ref cursor) = self.cursor {
            let separator = if self.params.is_empty() { '?' } else { '&' };
            format!("{}{}cursor={}", self.base_url, separator, urlencoding::encode(cursor))
        } else if self.params.is_empty() {
            self.base_url.clone()
        } else {
            format!("{}?{}", self.base_url, self.params)
        };

        // If we have a cursor and existing params, we need to append cursor
        if self.cursor.is_some() && !self.params.is_empty() {
            url = format!("{}?{}&cursor={}", self.base_url, self.params, urlencoding::encode(self.cursor.as_ref().unwrap()));
        }

        let page: T = self.client.get(&url).await?;

        let next_cursor = (self.extract_cursor)(&page);
        if next_cursor.is_none() || next_cursor.as_deref() == Some("") {
            self.exhausted = true;
        } else {
            self.cursor = next_cursor;
        }

        Ok(Some(page))
    }

    pub async fn collect_all(mut self) -> Result<Vec<T>> {
        let mut pages = Vec::new();
        while let Some(page) = self.next_page().await? {
            pages.push(page);
        }
        Ok(pages)
    }
}

pub struct PaginatedStreamOwned<T> {
    client: Arc<Intel471Client>,
    base_url: String,
    params: String,
    cursor: Option<String>,
    exhausted: bool,
    extract_cursor: CursorExtractor<T>,
}

impl<T> PaginatedStreamOwned<T> {
    pub fn new(
        client: Arc<Intel471Client>,
        base_url: String,
        params: String,
        extract_cursor: CursorExtractor<T>,
    ) -> Self {
        Self {
            client,
            base_url,
            params,
            cursor: None,
            exhausted: false,
            extract_cursor,
        }
    }
}

impl<T: serde::de::DeserializeOwned + Send + 'static> PaginatedStreamOwned<T> {
    pub async fn next_page(&mut self) -> Result<Option<T>> {
        if self.exhausted {
            return Ok(None);
        }

        let url = if let Some(ref cursor) = self.cursor {
            let separator = if self.params.is_empty() { '?' } else { '&' };
            format!("{}?{}{}cursor={}", self.base_url, self.params, separator, urlencoding::encode(cursor))
        } else if self.params.is_empty() {
            self.base_url.clone()
        } else {
            format!("{}?{}", self.base_url, self.params)
        };

        let page: T = self.client.get(&url).await?;

        let next_cursor = (self.extract_cursor)(&page);
        if next_cursor.is_none() || next_cursor.as_deref() == Some("") {
            self.exhausted = true;
        } else {
            self.cursor = next_cursor;
        }

        Ok(Some(page))
    }

    pub async fn collect_all(mut self) -> Result<Vec<T>> {
        let mut pages = Vec::new();
        while let Some(page) = self.next_page().await? {
            pages.push(page);
        }
        Ok(pages)
    }
}

pub fn build_query(params: &[(&str, Option<String>)]) -> String {
    params
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|val| format!("{}={}", k, urlencoding::encode(val))))
        .collect::<Vec<_>>()
        .join("&")
}

pub fn build_query_with_int(params: &[(&str, Option<String>)], int_params: &[(&str, Option<i64>)]) -> String {
    let mut parts: Vec<String> = params
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|val| format!("{}={}", k, urlencoding::encode(val))))
        .collect();

    for (k, v) in int_params {
        if let Some(val) = v {
            parts.push(format!("{}={}", k, val));
        }
    }

    parts.join("&")
}