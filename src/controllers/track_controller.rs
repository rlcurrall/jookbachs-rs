use actix_web::{get, web, Responder};

#[get("/api/tracks")]
pub async fn get_all_tracks() -> impl Responder {
    format!("All tracks...")
}

#[get("/api/tracks/{id}")]
pub async fn get_track_by_id(id: web::Path<u32>) -> impl Responder {
    format!("Track with id: {}", id)
}

#[get("/api/tracks/{id}/stream")]
pub async fn stream_track(id: web::Path<u32>) -> impl Responder {
    format!("Stream track with id: {}", id)
}
