use crate::error::Error;
use actix_web::{get, web, HttpResponse};
use handlebars::Handlebars;

#[get("mod/testmod")]
pub async fn mod_page_get(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse, Error> {
    let data = json!({
        "name": "Handlebars"
    });
    let body = hb.render("mod-page", &data)?;

    Ok(HttpResponse::Ok().body(body))
}
