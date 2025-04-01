use actix_web::{App, HttpServer};
use dotenv::dotenv;
use mafia_engine::routes::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = match std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
    {
        Ok(port) => port,
        Err(_) => panic!("PORT value is not a valid u16"),
    };

    println!("Starting server at http://{}:{}", host, port);

    HttpServer::new(|| App::new().configure(router::config_general_routes))
        .bind((host, port))?
        .run()
        .await
}
