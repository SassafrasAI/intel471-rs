use crate::client::Intel471Client;
use crate::error::Result;
use crate::models::actors::{ActorStreamPage, ActorStreamRequest};
use crate::pagination::{PaginatedStream, build_query_with_int};

const SERVICE_PATH: &str = "integrations/actors/v1";

#[derive(Debug, Clone)]
pub struct ActorsApi<'a> {
    pub(crate) client: &'a Intel471Client,
}

impl<'a> ActorsApi<'a> {
    pub fn stream(&self, request: ActorStreamRequest) -> Result<PaginatedStream<'a, ActorStreamPage>> {
        let url = self.client.url(SERVICE_PATH);
        let base_url = format!("{}/actors/stream", url);

        let params = build_query_with_int(
            &[
                ("actor", Some(request.actor.clone())),
                ("forum", request.forum.clone()),
                ("server_type", request.server_type.as_ref().map(|s| {
                    let s = serde_json::to_value(s).unwrap();
                    s.as_str().unwrap().to_string()
                })),
                ("cursor", request.cursor.clone()),
            ],
            &[
                ("from", request.from),
                ("until", request.until),
                ("size", request.size.map(|s| s as i64)),
            ],
        );

        Ok(PaginatedStream::new(
            self.client,
            base_url,
            params,
            |page: &ActorStreamPage| page.cursor_next.clone(),
        ))
    }

    pub async fn stream_page(&self, request: ActorStreamRequest) -> Result<ActorStreamPage> {
        let url = self.client.url(SERVICE_PATH);
        let mut params = vec![
            ("actor", Some(request.actor.clone())),
        ];

        if let Some(ref v) = request.forum {
            params.push(("forum", Some(v.clone())));
        }
        if let Some(ref v) = request.server_type {
            let s = serde_json::to_value(v).unwrap();
            params.push(("server_type", Some(s.as_str().unwrap().to_string())));
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
            format!("{}/actors/stream", url)
        } else {
            format!("{}/actors/stream?{}", url, query)
        };

        self.client.get(&full_url).await
    }
}