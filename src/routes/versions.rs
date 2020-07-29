use crate::error::Error;
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct Versions {
    release: Vec<String>,
    snapshot: Vec<String>,
    archaic: Vec<String>,
}

#[derive(Deserialize)]
struct InputFormat<'a> {
    // latest: LatestFormat,
    versions: Vec<VersionFormat<'a>>,
}
#[derive(Deserialize)]
struct VersionFormat<'a> {
    id: String,
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
}
// #[derive(Deserialize)]
// struct LatestFormat {
//     release: String,
//     snapshot: String,
// }

impl Versions {
    pub async fn get() -> Result<Versions, reqwest::Error> {
        let mut new = Versions {
            release: Vec::new(),
            snapshot: Vec::new(),
            archaic: Vec::new(),
        };
        let input = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
            .await?
            .json::<InputFormat>()
            .await?;

        for version in input.versions.into_iter() {
            let name = version.id;
            match &*version.type_ {
                "snapshot" => new.snapshot.push(name),
                "release" => new.release.push(name),
                "old_alpha" | "old_beta" => new.archaic.push(name),
                _ => (),
            }
        }

        Ok(new)
    }
}

#[get("versions/all.json")]
pub async fn versions_get(
    versions: web::Data<tokio::sync::RwLock<Versions>>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(&*versions.read().await))
}

#[get("versions/release.json")]
pub async fn versions_release(
    versions: web::Data<tokio::sync::RwLock<Versions>>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(&*versions.read().await.release))
}
#[get("versions/snapshot.json")]
pub async fn versions_snapshot(
    versions: web::Data<tokio::sync::RwLock<Versions>>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(&*versions.read().await.snapshot))
}
#[get("versions/archaic.json")]
pub async fn versions_archaic(
    versions: web::Data<tokio::sync::RwLock<Versions>>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(&*versions.read().await.archaic))
}

// https://launchermeta.mojang.com/mc/game/version_manifest.json
