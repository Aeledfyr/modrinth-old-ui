use crate::error::Error;
use actix_web::{get, web, web::Data, HttpResponse};
use handlebars::*;
use serde::{Deserialize, Serialize};

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
) -> Result<HttpResponse, Error> {
    match search(&*client, &info).await {
        Ok(mods) => {
            let data = json!({
                "query": info,
                "results": mods,
            });
            let body = hb.render("search-results", &data)?;
            Ok(HttpResponse::Ok().body(body))
        }
        Err(error) => {
            let data = json!({
                "query": info,
                "error": error.to_string(),
            });
            let body = hb.render("search-error", &data)?;
            Ok(HttpResponse::InternalServerError().body(body))
        }
    }
}

#[get("search")]
pub async fn search_get(
    client: web::Data<reqwest::Client>,
    web::Query(info): web::Query<SearchRequest>,
    hb: Data<Handlebars<'_>>,
) -> Result<HttpResponse, Error> {
    let analytics = dotenv::var("ENABLE_ANALYTICS")
        .ok()
        .map(|v| v.parse::<bool>().unwrap())
        .unwrap_or(false);

    match search(&*client, &info).await {
        Ok(mods) => {
            let data = json!({
                "query": info,
                "results": mods,
                "analytics": analytics,
            });

            let body = hb.render("search", &data)?;
            Ok(HttpResponse::Ok().body(body))
        }
        Err(error) => {
            let data = json!({
                "query": info,
                "results": [],
                "analytics": analytics,
                "error": error.to_string(),
            });

            let body = hb.render("search", &data)?;

            Ok(HttpResponse::InternalServerError().body(body))
        }
    }
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

async fn search(client: &reqwest::Client, info: &SearchRequest) -> Result<Vec<SearchMod>, Error> {
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
        let mut mods = body.json::<Vec<SearchMod>>().await?;
        for m in &mut mods {
            if m.icon_url.is_empty() {
                m.icon_url = String::from("/static/images/icon/missing.svg");
            }
            // Convert ISO date to YYYY-MM-DD
            m.date_created.truncate(10);
            m.date_modified.truncate(10);
        }
        Ok(mods)
    } else {
        Err(Error::ApiError(body.json().await?))
    }
}
