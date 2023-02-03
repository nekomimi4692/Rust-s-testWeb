use std::collections::HashMap;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
    title: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = if let Some(name) = query.get("name") {
        UserTemplate {
            name,
            text: "Welcome!",
            title: "test app",
        }
        .render()
        .unwrap()
    } else {
        Index.render().unwrap()
    };
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

/*
#[derive(Template)]
#[template(path = "home.html")]
struct Home;

async fn home(username: web::Query) -> std::io::Result<()> {
    let h = username.get(name)
    match h {

    }
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
  
    // start http server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            //.service(web::resource("/home").route(web::get().to(        )))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
