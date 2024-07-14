mod config;
mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_env();
    let pool = db::establish_connection(&config.database_url).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/parcels", web::post().to(handlers::add_parcel))
            .route("/parcels/country/{country}", web::get().to(handlers::filter_parcels_by_country))
            .route("/parcels/description/{description}", web::get().to(handlers::search_parcels_by_description))
    })
    .bind("0.0.0.0:8080")? // Bind to all available IP addresses on port 8080
    .run()
    .await
}
