use actix_web::{get, web, web::Data, HttpResponse};
use handlebars::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("Error connecting to backend")]
    ConnectionError(#[from] reqwest::Error),
    #[error("API error: {0}")]
    ApiError(serde_json::Value),
    #[error("Error rendering page")]
    RenderError(#[from] handlebars::RenderError),
}

use actix_web::http::StatusCode;
impl actix_web::ResponseError for SearchError {
    fn status_code(&self) -> StatusCode {
        match self {
            SearchError::ConnectionError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            SearchError::ApiError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            SearchError::RenderError(..) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string()) // TODO: error page
    }
}

#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    #[serde(rename = "q")]
    query: Option<String>,
    #[serde(rename = "f")]
    filters: Option<String>,
    #[serde(rename = "a")]
    facets: Option<String>,
    #[serde(rename = "v")]
    version: Option<String>,
    #[serde(rename = "o")]
    offset: Option<String>,
    #[serde(rename = "s")]
    index: Option<String>,
}

#[get("search_live")]
pub async fn search_live(
    client: web::Data<reqwest::Client>,
    web::Query(info): web::Query<SearchRequest>,
    hb: Data<Handlebars<'_>>,
) -> HttpResponse {
    let results = search(&*client, &info).await.unwrap();
    let data = json!({
        "query": info,
        "results": results,
    });
    let body = hb.render("search-results", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("search")]
pub async fn search_get(
    client: web::Data<reqwest::Client>,
    web::Query(info): web::Query<SearchRequest>,
    hb: Data<Handlebars<'_>>,
) -> Result<HttpResponse, SearchError> {
    let results = search(&*client, &info).await?;

    let data = json!({
        "query": info,
        "results": results,
    });

    let body = hb.render("search", &data)?;

    Ok(HttpResponse::Ok().body(body))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SearchMod {
    mod_id: String,
    author: String,
    title: String,
    description: String,
    categories: Vec<String>,
    versions: Vec<String>,
    downloads: i32,
    page_url: String,
    icon_url: String,
    author_url: String,
    date_created: String,
    date_modified: String,
    latest_version: String,
}

async fn search(
    client: &reqwest::Client,
    info: &SearchRequest,
) -> Result<Vec<SearchMod>, SearchError> {
    let query = [
        ("query", info.query.as_ref()),
        ("filters", info.filters.as_ref()),
        ("facets", info.facets.as_ref()),
        ("version", info.version.as_ref()),
        ("offset", info.offset.as_ref()),
        ("index", info.index.as_ref()),
    ];
    let body = client
        .get("http://localhost:8000/api/v1/mod")
        .query(&query)
        .send()
        .await?;

    if body.status().is_success() {
        Ok(body.json().await?)
    } else {
        Err(SearchError::ApiError(body.json().await?))
    }
}
