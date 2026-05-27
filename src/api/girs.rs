use crate::client::Intel471Client;
use crate::error::Result;
use crate::models::girs::GirsTreeResponse;

const SERVICE_PATH: &str = "integrations/girs/v1";

#[derive(Debug, Clone)]
pub struct GirsApi<'a> {
    pub(crate) client: &'a Intel471Client,
}

impl<'a> GirsApi<'a> {
    pub async fn tree(&self) -> Result<GirsTreeResponse> {
        let url = self.client.url(SERVICE_PATH);
        let full_url = format!("{}/girs/tree", url);
        self.client.get(&full_url).await
    }
}