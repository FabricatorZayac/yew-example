use gloo_net::http::{Request, Response};
use serde::{de::DeserializeOwned, Serialize};

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(gloo_net::Error),
}

pub async fn get(url: &str) -> Result<Response, gloo_net::Error> {
    Request::get(url).send().await
}

pub async fn get_object<T>(url: &str) -> Result<T, gloo_net::Error>
where
    T: Serialize + DeserializeOwned,
{
    let response_str = get(url).await?.text().await?;
    Ok(serde_json::from_str(&response_str)?)
}

pub async fn post<T>(url: &str, body: T) -> Result<Response, gloo_net::Error>
where
    T: Serialize,
{
    Request::post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await
}

pub async fn delete(url: &str) -> Result<Response, gloo_net::Error> {
    Request::delete(url).send().await
}
