use crate::constants;
use crate::Result;
use hyper::client::Client;
use hyper::header::HeaderValue;
use hyper::{Body, Method, Request};
use hyper_tls::HttpsConnector;
use serde::Serialize;
use tokio::runtime::Runtime;

/**
http post request
*/
pub fn post2<T: Serialize>(action: &str, req_data: &T) -> reqwest::Response {
    let url: String = constants::URL.to_string() + action;
    let client = reqwest::Client::new();
    let response: reqwest::Response = client.post(&*url).json(&req_data).send().unwrap();
    response
}

pub fn post(action: &str, req_data: Vec<u8>) -> Result<String> {
    let f = async_post(action, req_data);
    Runtime::new().unwrap().block_on(f)
}

async fn async_post(action: &str, req_data: Vec<u8>) -> Result<String> {
    let uri: hyper::Uri = format!("{}{}", constants::URL.to_string(), action)
        .to_string()
        .parse()
        .unwrap();
    let mut req = Request::new(Body::from(req_data));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let resp = client.request(req).await?;

    let bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let res_data = std::str::from_utf8(&bytes).unwrap().to_string();
    Ok(res_data)
}
