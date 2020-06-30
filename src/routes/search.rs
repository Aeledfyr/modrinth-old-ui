use actix_web::{get, web, web::Data, HttpResponse};
use handlebars::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    #[serde(rename = "q")]
    query: Option<String>,
    #[serde(rename = "f")]
    filters: Option<String>,
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
) -> HttpResponse {
    let results = search(&*client, &info).await.unwrap();

    let data = json!({
        "query": info,
        "results": results,
    });

    let body = hb.render("search", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SearchMod {
    mod_id: i32,
    author: String,
    title: String,
    description: String,
    keywords: Vec<String>,
    versions: Vec<String>,
    downloads: i32,
    page_url: String,
    icon_url: String,
    author_url: String,
    date_created: String,
    created: i64,
    date_modified: String,
    updated: i64,
    latest_version: String,
    empty: String,
}

async fn search(
    client: &reqwest::Client,
    info: &SearchRequest,
) -> Result<Vec<SearchMod>, Box<dyn std::error::Error>> {
    let query = [
        ("query", info.query.as_ref()),
        ("filters", info.filters.as_ref()),
        ("version", info.version.as_ref()),
        ("offset", info.offset.as_ref()),
        ("index", info.index.as_ref()),
    ];
    let mods = client
        .get("http://localhost:8000/api/v1/mods")
        .query(&query)
        .send()
        .await?
        .json::<Vec<SearchMod>>()
        .await?;
    Ok(mods)
}
