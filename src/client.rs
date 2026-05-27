use crate::api::{
    actors::ActorsApi, ase::AseApi, brand_exposure::BrandExposureApi, credentials::CredentialsApi,
    entities::EntitiesApi, girs::GirsApi, indicators::IndicatorsApi, malware::MalwareApi,
    observables::ObservablesApi, reports::ReportsApi, sources::SourcesApi, tprm::TprmApi,
    watchers::WatchersApi,
};

const BASE_URL: &str = "https://api.intel471.cloud";

#[derive(Debug, Clone)]
pub struct Intel471Client {
    inner: reqwest::Client,
    api_key_username: String,
    api_key_password: String,
    base_url: String,
}

#[allow(elided_lifetimes_in_paths)]
impl Intel471Client {
    pub fn new(api_key_username: impl Into<String>, api_key_password: impl Into<String>) -> Self {
        Self::with_base_url(api_key_username, api_key_password, BASE_URL)
    }

    pub fn with_base_url(
        api_key_username: impl Into<String>,
        api_key_password: impl Into<String>,
        base_url: &str,
    ) -> Self {
        Self {
            inner: reqwest::Client::new(),
            api_key_username: api_key_username.into(),
            api_key_password: api_key_password.into(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub fn actors(&self) -> ActorsApi<'_> {
        ActorsApi { client: self }
    }

    pub fn ase(&self) -> AseApi<'_> {
        AseApi { client: self }
    }

    pub fn brand_exposure(&self) -> BrandExposureApi<'_> {
        BrandExposureApi { client: self }
    }

    pub fn credentials(&self) -> CredentialsApi<'_> {
        CredentialsApi { client: self }
    }

    pub fn entities(&self) -> EntitiesApi<'_> {
        EntitiesApi { client: self }
    }

    pub fn girs(&self) -> GirsApi<'_> {
        GirsApi { client: self }
    }

    pub fn indicators(&self) -> IndicatorsApi<'_> {
        IndicatorsApi { client: self }
    }

    pub fn malware(&self) -> MalwareApi<'_> {
        MalwareApi { client: self }
    }

    pub fn observables(&self) -> ObservablesApi<'_> {
        ObservablesApi { client: self }
    }

    pub fn reports(&self) -> ReportsApi<'_> {
        ReportsApi { client: self }
    }

    pub fn sources(&self) -> SourcesApi<'_> {
        SourcesApi { client: self }
    }

    pub fn tprm(&self) -> TprmApi<'_> {
        TprmApi { client: self }
    }

    pub fn watchers(&self) -> WatchersApi<'_> {
        WatchersApi { client: self }
    }

    pub(crate) async fn get<T: serde::de::DeserializeOwned>(&self, url: &str) -> crate::error::Result<T> {
        let resp = self
            .inner
            .get(url)
            .basic_auth(&self.api_key_username, Some(&self.api_key_password))
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::Error::Api {
                status: status.as_u16(),
                message: body,
                timestamp: None,
            });
        }

        let data: T = resp.json().await?;
        Ok(data)
    }

    #[allow(dead_code)]
    pub(crate) async fn get_with_bearer<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        token: &str,
    ) -> crate::error::Result<T> {
        let resp = self
            .inner
            .get(url)
            .bearer_auth(token)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::Error::Api {
                status: status.as_u16(),
                message: body,
                timestamp: None,
            });
        }

        let data: T = resp.json().await?;
        Ok(data)
    }

    pub(crate) async fn get_raw(&self, url: &str) -> crate::error::Result<reqwest::Response> {
        let resp = self
            .inner
            .get(url)
            .basic_auth(&self.api_key_username, Some(&self.api_key_password))
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::Error::Api {
                status: status.as_u16(),
                message: body,
                timestamp: None,
            });
        }

        Ok(resp)
    }

    pub(crate) async fn post<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        url: &str,
        body: &B,
    ) -> crate::error::Result<T> {
        let resp = self
            .inner
            .post(url)
            .basic_auth(&self.api_key_username, Some(&self.api_key_password))
            .json(body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::Error::Api {
                status: status.as_u16(),
                message: body,
                timestamp: None,
            });
        }

        let data: T = resp.json().await?;
        Ok(data)
    }

    pub(crate) async fn put_void<B: serde::Serialize>(
        &self,
        url: &str,
        body: &B,
    ) -> crate::error::Result<()> {
        let resp = self
            .inner
            .put(url)
            .basic_auth(&self.api_key_username, Some(&self.api_key_password))
            .json(body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::Error::Api {
                status: status.as_u16(),
                message: body,
                timestamp: None,
            });
        }

        Ok(())
    }

    pub(crate) async fn put_with_bearer_void<B: serde::Serialize>(
        &self,
        url: &str,
        token: &str,
        body: &B,
    ) -> crate::error::Result<()> {
        let resp = self
            .inner
            .put(url)
            .bearer_auth(token)
            .json(body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::Error::Api {
                status: status.as_u16(),
                message: body,
                timestamp: None,
            });
        }

        Ok(())
    }

    pub(crate) async fn patch_void<B: serde::Serialize>(
        &self,
        url: &str,
        body: &B,
    ) -> crate::error::Result<()> {
        let resp = self
            .inner
            .patch(url)
            .basic_auth(&self.api_key_username, Some(&self.api_key_password))
            .json(body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::Error::Api {
                status: status.as_u16(),
                message: body,
                timestamp: None,
            });
        }

        Ok(())
    }

    pub(crate) async fn post_void<B: serde::Serialize>(
        &self,
        url: &str,
        body: &B,
    ) -> crate::error::Result<()> {
        let resp = self
            .inner
            .post(url)
            .basic_auth(&self.api_key_username, Some(&self.api_key_password))
            .json(body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::Error::Api {
                status: status.as_u16(),
                message: body,
                timestamp: None,
            });
        }

        Ok(())
    }

    pub(crate) async fn post_void_empty(&self, url: &str) -> crate::error::Result<()> {
        let resp = self
            .inner
            .post(url)
            .basic_auth(&self.api_key_username, Some(&self.api_key_password))
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::Error::Api {
                status: status.as_u16(),
                message: body,
                timestamp: None,
            });
        }

        Ok(())
    }

    pub(crate) async fn delete_void(&self, url: &str) -> crate::error::Result<()> {
        let resp = self
            .inner
            .delete(url)
            .basic_auth(&self.api_key_username, Some(&self.api_key_password))
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::Error::Api {
                status: status.as_u16(),
                message: body,
                timestamp: None,
            });
        }

        Ok(())
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path)
    }
}