use crate::Result;
use client::network;
use reqwest::IntoUrl;

/// Queries url for the etag header
/// Note: Will default to `MISSING_ETAG` incase header isn't found
pub(crate) async fn query_etag<U: IntoUrl>(url: U) -> Result<String> {
    Ok(network::WEB_CLIENT.head(url).send().await?.headers()
        .get("etag")
        .map(|x| x.to_str().unwrap().to_string()) // Etag will always be a valid UTF-8 due to it being ASCII
        .unwrap_or("MISSING_ETAG".into()))
}

/// Extracts Etag value from response
/// Note: Will default to `MISSING_ETAG` incase header isn't found
pub(crate) fn get_etag(x: &reqwest::Response) -> String {
    x.headers().get("etag").map(|x| x.to_str().unwrap().to_string()) // Etag will always be a valid UTF-8 due to it being ASCII
        .unwrap_or("MISSING_ETAG".into())
}

pub(crate) async fn query<U: IntoUrl>(url: U) -> Result<reqwest::Response> {
    Ok(network::WEB_CLIENT.get(url).send().await?)
}
