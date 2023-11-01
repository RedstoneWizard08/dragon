use anyhow::Result;
use axum::http::HeaderValue;
use hyper::{body::Bytes, HeaderMap, Method};
use reqwest::{Client, Response};

#[derive(Debug, Clone)]
pub struct ProxyState {
    /// The base URL. Must be `http(s)://[domain]:[port]`.
    /// This MUST not have a trailing slash.
    pub(crate) base: String,

    /// The reqwest client.
    pub(crate) client: Client,
}

impl ProxyState {
    pub fn new(base: String) -> Self {
        Self {
            base,
            client: Client::new(),
        }
    }

    pub async fn request<T>(
        &self,
        method: Method,
        url: T,
        query: Option<&[(&str, &str)]>,
        body: Option<Bytes>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> Result<Response>
    where
        T: AsRef<str>,
    {
        let url = format!("{}{}", self.base, url.as_ref());
        let mut builder = self.client.request(method, url);

        if let Some(headers) = headers {
            builder = builder.headers(headers);
        }

        if let Some(query) = query {
            let query = query
                .iter()
                .map(|(a, b)| (a.as_ref(), b.as_ref()))
                .collect::<Vec<(&str, &str)>>();

            builder = builder.query(&query);
        }

        if let Some(body) = body {
            builder = builder.body(body);
        }

        builder.send().await.map_err(|v| anyhow!(v))
    }

    pub async fn get<T>(
        &self,
        url: T,
        query: Option<&[(&str, &str)]>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> Result<Response>
    where
        T: AsRef<str>,
    {
        self.request(Method::GET, url, query, None, headers).await
    }

    pub async fn post<T>(
        &self,
        url: T,
        query: Option<&[(&str, &str)]>,
        body: Option<Bytes>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> Result<Response>
    where
        T: AsRef<str>,
    {
        self.request(Method::POST, url, query, body, headers).await
    }

    pub async fn put<T>(
        &self,
        url: T,
        query: Option<&[(&str, &str)]>,
        body: Option<Bytes>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> Result<Response>
    where
        T: AsRef<str>,
    {
        self.request(Method::PUT, url, query, body, headers).await
    }

    pub async fn patch<T>(
        &self,
        url: T,
        query: Option<&[(&str, &str)]>,
        body: Option<Bytes>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> Result<Response>
    where
        T: AsRef<str>,
    {
        self.request(Method::PATCH, url, query, body, headers).await
    }

    pub async fn delete<T>(
        &self,
        url: T,
        query: Option<&[(&str, &str)]>,
        body: Option<Bytes>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> Result<Response>
    where
        T: AsRef<str>,
    {
        self.request(Method::DELETE, url, query, body, headers)
            .await
    }
}
