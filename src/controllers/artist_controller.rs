use actix_files::NamedFile;
use actix_web::{
    get,
    web::{HttpRequest, HttpResponse, Path},
    Responder,
};
// use metaflac::Tag;
use id3::Tag;
use serde_json::json;

// get all albums
#[get("/api/artists")]
pub async fn get_all_artists(req: HttpRequest) -> impl Responder {
    let file = NamedFile::open("/home/robb/Music/20 Years Gone.mp3")?;
    file.into_response(&req)
}

// get album by id
#[get("/api/artists/{id}")]
pub async fn get_artist_by_id(id: Path<u32>) -> impl Responder {
    let md = Tag::read_from_path("/home/robb/Music/20 Years Gone.mp3").unwrap();
    HttpResponse::Ok().json(json!({
        "id": id.into_inner(),
        "title": md.title().unwrap(),
        "artist": md.artist().unwrap(),
        "album": md.album().unwrap(),
        "year": md.year().unwrap(),
    }))
}

// get album tracks
#[get("/api/artists/{id}/tracks")]
pub async fn get_artist_tracks(id: Path<u32>) -> impl Responder {
    format!("Tracks for album with id: {}", id)
}
