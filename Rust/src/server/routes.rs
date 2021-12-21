use serde_json::json;
use warp::{ Rejection, Reply};


async fn health() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&json!({ "ok": true })))
}