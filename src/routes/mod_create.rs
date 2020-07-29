use actix_web::{get, web, HttpResponse};
use handlebars::Handlebars;
use crate::error::Error;

#[get("createmod")]
pub async fn mod_create_get(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse, Error> {
    let data = json!({
        "name": "Handlebars"
    });
    let body = hb.render("mod-create", &data)?;

    Ok(HttpResponse::Ok().body(body))
}
