use actix_web::{get, web::Path, Responder};

// get all albums
#[get("/api/artists")]
pub async fn get_all_artists() -> impl Responder {
    format!("All artists...")
}

// get album by id
#[get("/api/artists/{id}")]
pub async fn get_artist_by_id(id: Path<u32>) -> impl Responder {
    format!("Artist with id: {}", id)
}

// get album tracks
#[get("/api/artists/{id}/tracks")]
pub async fn get_artist_tracks(id: Path<u32>) -> impl Responder {
    format!("Tracks for album with id: {}", id)
}
