// Hand-crafted async HTTP client for the UniFi Network Integration API (v10.1.84).
//
// Base path: /integration/v1/
// Auth: X-API-KEY header

use std::future::Future;

use reqwest::header::{HeaderMap, HeaderValue};
use secrecy::ExposeSecret;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tracing::debug;
use url::Url;

use super::types;
use crate::Error;

mod clients;
mod devices;
mod firewall;
mod networks;
mod policy;
mod reference;
mod system;
mod wifi;

// ── Error response shape from the Integration API ────────────────────

#[derive(serde::Deserialize)]
struct ErrorResponse {
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    code: Option<String>,
}

// ── Client ───────────────────────────────────────────────────────────

/// Async client for the UniFi Integration API.
///
/// Uses API-key authentication and communicates via JSON REST endpoints
/// under `/integration/v1/`.
pub struct IntegrationClient {
    http: reqwest::Client,
    base_url: Url,
}

impl IntegrationClient {
    // ── Constructors ─────────────────────────────────────────────────

    /// Build from an API key, transport config, and detected platform.
    ///
    /// Injects `X-API-KEY` as a default header on every request.
    /// On UniFi OS the base path is `/proxy/network/integration/`;
    /// on standalone controllers it's just `/integration/`.
    pub fn from_api_key(
        base_url: &str,
        api_key: &secrecy::SecretString,
        transport: &crate::TransportConfig,
        platform: crate::ControllerPlatform,
    ) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        let mut key_value =
            HeaderValue::from_str(api_key.expose_secret()).map_err(|e| Error::Authentication {
                message: format!("invalid API key header value: {e}"),
            })?;
        key_value.set_sensitive(true);
        headers.insert("X-API-KEY", key_value);

        let http = transport.build_client_with_headers(headers)?;
        let base_url = Self::normalize_base_url(base_url, platform)?;

        Ok(Self { http, base_url })
    }

    /// Wrap an existing `reqwest::Client` (caller manages auth headers).
    pub fn from_reqwest(
        base_url: &str,
        http: reqwest::Client,
        platform: crate::ControllerPlatform,
    ) -> Result<Self, Error> {
        let base_url = Self::normalize_base_url(base_url, platform)?;
        Ok(Self { http, base_url })
    }

    /// Build the base URL with correct platform prefix + `/integration/`.
    ///
    /// UniFi OS: `https://host/proxy/network/integration/`
    /// Standalone: `https://host/integration/`
    fn normalize_base_url(raw: &str, platform: crate::ControllerPlatform) -> Result<Url, Error> {
        let mut url = Url::parse(raw)?;

        // Strip trailing slash for uniform handling
        let path = url.path().trim_end_matches('/').to_owned();

        if path.ends_with("/integration") {
            url.set_path(&format!("{path}/"));
        } else {
            let prefix = platform.integration_prefix();
            url.set_path(&format!("{path}{prefix}/"));
        }

        Ok(url)
    }

    // ── URL builder ──────────────────────────────────────────────────

    /// Join a relative path (e.g. `"v1/sites"`) onto the base URL.
    fn url(&self, path: &str) -> Url {
        // base_url always ends with `/integration/`, so joining `v1/…` works.
        self.base_url
            .join(path)
            .expect("path should be valid relative URL")
    }

    // ── HTTP verbs ───────────────────────────────────────────────────

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = self.url(path);
        debug!("GET {url}");

        let resp = self.http.get(url).send().await?;
        self.handle_response(resp).await
    }

    async fn get_with_params<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<T, Error> {
        let url = self.url(path);
        debug!("GET {url} params={params:?}");

        let resp = self.http.get(url).query(params).send().await?;
        self.handle_response(resp).await
    }

    async fn post<T: DeserializeOwned, B: Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let url = self.url(path);
        debug!("POST {url}");

        let resp = self.http.post(url).json(body).send().await?;
        self.handle_response(resp).await
    }

    async fn post_no_response<B: Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), Error> {
        let url = self.url(path);
        debug!("POST {url}");

        let resp = self.http.post(url).json(body).send().await?;
        self.handle_empty(resp).await
    }

    async fn put<T: DeserializeOwned, B: Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let url = self.url(path);
        debug!("PUT {url}");

        let resp = self.http.put(url).json(body).send().await?;
        self.handle_response(resp).await
    }

    async fn patch<T: DeserializeOwned, B: Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let url = self.url(path);
        debug!("PATCH {url}");

        let resp = self.http.patch(url).json(body).send().await?;
        self.handle_response(resp).await
    }

    async fn delete(&self, path: &str) -> Result<(), Error> {
        let url = self.url(path);
        debug!("DELETE {url}");

        let resp = self.http.delete(url).send().await?;
        self.handle_empty(resp).await
    }

    async fn delete_with_response<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = self.url(path);
        debug!("DELETE {url}");

        let resp = self.http.delete(url).send().await?;
        self.handle_response(resp).await
    }

    async fn delete_with_params<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<T, Error> {
        let url = self.url(path);
        debug!("DELETE {url} params={params:?}");

        let resp = self.http.delete(url).query(params).send().await?;
        self.handle_response(resp).await
    }

    // ── Response handling ────────────────────────────────────────────

    async fn handle_response<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, Error> {
        let status = resp.status();
        if status.is_success() {
            let body = resp.text().await?;
            serde_json::from_str(&body).map_err(|e| {
                let preview = &body[..body.len().min(200)];
                Error::Deserialization {
                    message: format!("{e} (body preview: {preview:?})"),
                    body,
                }
            })
        } else {
            Err(self.parse_error(status, resp).await)
        }
    }

    async fn handle_empty(&self, resp: reqwest::Response) -> Result<(), Error> {
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(self.parse_error(status, resp).await)
        }
    }

    async fn parse_error(&self, status: reqwest::StatusCode, resp: reqwest::Response) -> Error {
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Error::InvalidApiKey;
        }

        let raw = resp.text().await.unwrap_or_default();

        if let Ok(err) = serde_json::from_str::<ErrorResponse>(&raw) {
            Error::Integration {
                status: status.as_u16(),
                message: err.message.unwrap_or_else(|| status.to_string()),
                code: err.code,
            }
        } else {
            Error::Integration {
                status: status.as_u16(),
                message: if raw.is_empty() {
                    status.to_string()
                } else {
                    raw
                },
                code: None,
            }
        }
    }

    // ── Pagination helper ────────────────────────────────────────────

    /// Collect all pages into a single `Vec<T>`.
    pub async fn paginate_all<T, F, Fut>(&self, limit: i32, fetch: F) -> Result<Vec<T>, Error>
    where
        F: Fn(i64, i32) -> Fut,
        Fut: Future<Output = Result<types::Page<T>, Error>>,
    {
        let mut all = Vec::new();
        let mut offset: i64 = 0;

        loop {
            let page = fetch(offset, limit).await?;
            let received = page.data.len();
            all.extend(page.data);

            let limit_usize = usize::try_from(limit).unwrap_or(0);
            if received < limit_usize
                || i64::try_from(all.len()).unwrap_or(i64::MAX) >= page.total_count
            {
                break;
            }

            offset += i64::try_from(received).unwrap_or(i64::MAX);
        }

        Ok(all)
    }
}
