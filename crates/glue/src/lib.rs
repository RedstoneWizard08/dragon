#![feature(async_closure)]

use axum::{debug_handler, http::Request, response::Response, routing::get, Extension, Router};
use hyper::{
    body::to_bytes,
    header::{CONTENT_LENGTH, CONTENT_TYPE, TRANSFER_ENCODING},
    Body, StatusCode,
};
use include_dir::Dir;
use state::ProxyState;
use ws::websocket_handler;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate log;

pub mod state;
pub mod ws;

#[debug_handler]
pub async fn fallback_handler(
    Extension(state): Extension<ProxyState>,
    mut req: Request<Body>,
) -> Response<Body> {
    let method = req.method_mut().clone();
    let path = req.uri_mut().clone();
    let path = path.path();
    let headers = req.headers_mut().clone();
    let body = req.body_mut();

    let res = state
        .request(
            method,
            path,
            None,
            Some(to_bytes(body).await.unwrap()),
            Some(headers),
        )
        .await
        .unwrap();

    let mut builder = Response::builder().status(res.status());

    for key in res.headers().keys() {
        if *key == CONTENT_LENGTH || *key == TRANSFER_ENCODING {
            continue;
        }

        builder = builder.header(key, res.headers().get(key).unwrap());
    }

    builder
        .body(Body::from(res.bytes().await.unwrap()))
        .unwrap()
}

/// Register the proxy handler.
/// Accepts a base and a router.
/// The base must be in the format "http(s)://[ip or domain]:[optional port]"
/// with NO trailing slash (or it will break).
pub fn register_proxy_handler(base: String, router: Router) -> Router {
    let state = ProxyState::new(base);

    router
        .route("/_next/webpack-hmr", get(websocket_handler))
        .fallback(fallback_handler)
        .layer(Extension(state))
}

#[debug_handler]
pub async fn handle_static(
    Extension(dir): Extension<Dir<'static>>,
    req: Request<Body>,
) -> Response<Body> {
    let path = req.uri().path();
    let index_path = format!("{}index.html", path);

    let path = if path.ends_with("/") {
        index_path.as_str()
    } else {
        path
    };

    let path = path.trim_start_matches("/");

    if let Some(file) = dir.get_file(path) {
        let mime = mime_guess::from_path(path).first().unwrap();

        return Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, mime.to_string())
            .body(Body::from(file.contents().to_vec()))
            .unwrap();
    }

    if let Some(file) = dir.get_file("404.html") {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(CONTENT_TYPE, "text/html")
            .body(Body::from(file.contents().to_vec()))
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(
            "Cannot find the file specified!".as_bytes().to_vec(),
        ))
        .unwrap()
}

pub fn register_static_handler(dir: Dir<'static>, router: Router) -> Router {
    router.fallback(handle_static).layer(Extension(dir))
}

#[derive(Debug, Clone)]
pub struct GlueOpts {
    pub base: Option<String>,
    pub dir: Option<Dir<'static>>,
}

impl GlueOpts {
    pub fn new() -> Self {
        Self {
            base: None,
            dir: None,
        }
    }

    pub fn base<T>(&mut self, base: T) -> Self
    where
        T: AsRef<str>,
    {
        self.base = Some(base.as_ref().to_string());

        self.clone()
    }

    pub fn dir(&mut self, dir: Option<Dir<'static>>) -> Self {
        self.dir = dir;

        self.clone()
    }
}

pub fn register_glue(router: Router, opts: GlueOpts) -> Router {
    cfg_if! {
        if #[cfg(debug_assertions)] {
            register_proxy_handler(opts.base.unwrap(), router)
        } else {
            register_static_handler(opts.dir.unwrap(), router)
        }
    }
}
