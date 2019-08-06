#![feature(async_await)]

use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Result};
use actix_web_async_compat::async_compat;
use futures03::{compat::Future01CompatExt as _, FutureExt as _, TryFutureExt as _};
use serde::Deserialize;
use std::{
    io,
    time::{Duration, Instant},
};
use futures::Future;

#[async_compat]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("OK"))
}

#[derive(Debug, Deserialize)]
struct UserForm {
    name: String,
}

#[async_compat]
async fn index2(form: actix_web::web::Form<UserForm>) -> Result<HttpResponse> {
    dbg!(form);

    Ok(HttpResponse::Ok().body("OK"))
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(web::resource("/").route(web::get().to_async(index)))
            .service(web::resource("/welcome2").route(web::post().to_async(index2)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}
