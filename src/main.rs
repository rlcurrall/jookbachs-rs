use actix_rt;
use actix_web::{App, HttpServer};

mod controllers;

use controllers::{album_controller, artist_controller, track_controller};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
    //.bind_uds("/tmp/jookbachs.sock")?
    .bind("localhost:8080")?
    .start()
    .await
}
