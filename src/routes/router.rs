use actix_files as fs;
use actix_web::{get, web, HttpResponse, Responder};
use maud::html;

#[get("/")]
async fn load_main_page() -> impl Responder {
    let markup = html! {
        html {
            body {
                h1 {
                    "Hello, world!"
                }
            }
        }
    };
    let html = markup.into_string();
    HttpResponse::Ok().body(html)
}

pub fn config_general_routes(config: &mut web::ServiceConfig) {
    config
        .service(load_main_page)
        .service(fs::Files::new("/public", "./public").show_files_listing());
}
