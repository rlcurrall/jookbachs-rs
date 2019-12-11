use actix_files::NamedFile;
use actix_web::{
    get,
    web::{HttpRequest, HttpResponse, Path},
    Responder,
};
use id3::Tag;
use serde_json::json;

#[get("/api/tracks")]
pub async fn get_all_tracks(req: HttpRequest) -> impl Responder {
    //format!("All tracks...")
    NamedFile::open("index.html")?.into_response(&req)
}

#[get("/api/tracks/{id}")]
pub async fn get_track_by_id(id: Path<u32>) -> impl Responder {
    //format!("Track with id: {}", id)
    let md = Tag::read_from_path("/home/robb/Music/20 Years Gone.mp3").unwrap();
    HttpResponse::Ok().json(json!({
        "id": id.into_inner(),
        "title": md.title().unwrap(),
        "artist": md.artist().unwrap(),
        "album": md.album().unwrap(),
        "year": md.year().unwrap(),
    }))
}

#[get("/api/tracks/{id}/stream")]
pub async fn stream_track(req: HttpRequest, _id: Path<u32>) -> impl Responder {
    //format!("Stream track with id: {}", id)
    NamedFile::open("/home/robb/Music/20 Years Gone.mp3")?.into_response(&req)
}
