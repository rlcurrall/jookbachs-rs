use actix_cors::Cors;
use actix_rt;
use actix_web::{App, HttpServer};

mod controllers;

use controllers::{album_controller, artist_controller, track_controller};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8000")
                    .allowed_methods(vec!["GET", "POST"])
                    .max_age(3600)
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
    .bind("127.0.0.1:8080")?
    .start()
    .await
}
