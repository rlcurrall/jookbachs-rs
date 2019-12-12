#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_rt;
use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod controllers;
mod env;
mod models;
mod schema;

use controllers::{album_controller, artist_controller, track_controller};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment variables
    env::init();

    // Create connection pool for database
    let conn_spec = std::env::var("DB_HOST").expect("DB_HOST");
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start application
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(
                Cors::new()
                    .allowed_origin(std::env::var("APP_HOST").unwrap().as_str())
                    .allowed_origin(std::env::var("UI_HOST").unwrap().as_str())
                    .supports_credentials()
                    .max_age(2500)
                    .finish(),
            )
            // Album Routes
            .service(album_controller::get_all_albums)
            .service(album_controller::get_album_by_id)
            .service(album_controller::get_album_tracks)
            // Artist Routes
            .service(artist_controller::get_all_artists)
            .service(artist_controller::get_artist_by_id)
            .service(artist_controller::get_artist_tracks)
            // Track Routes
            .service(track_controller::get_all_tracks)
            .service(track_controller::get_track_by_id)
            .service(track_controller::stream_track)
    })
    .bind(std::env::var("APP_HOST").unwrap().as_str())?
    .start()
    .await
}
