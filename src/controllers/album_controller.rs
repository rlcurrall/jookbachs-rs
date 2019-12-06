use actix_web::{
    get,
    web::{HttpResponse, Path},
    Responder,
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct Person {
    id: u32,
    name: String,
    age: u32,
}

#[get("/api/albums")]
pub async fn get_all_albums() -> impl Responder {
    HttpResponse::Ok().json(json!([
        {
            "name": "Robb",
            "age": 27
        }
    ]))
}

#[get("/api/albums/{id}")]
pub async fn get_album_by_id(id: Path<u32>) -> impl Responder {
    let user = Person {
        id: id.into_inner(),
        name: String::from("Robb"),
        age: 27,
    };

    HttpResponse::Ok().json(user)
}

#[get("/api/albums/{id}/tracks")]
pub async fn get_album_tracks(id: Path<u32>) -> impl Responder {
    format!("Tracks for album with id: {}", id)
}
