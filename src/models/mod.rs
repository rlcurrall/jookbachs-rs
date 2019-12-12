use crate::schema::{albums, artists, tracks};
use chrono::{offset::Utc, DateTime};

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "artists"]
pub struct Artist {
    id: u32,
    name: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Artist)]
#[table_name = "albums"]
pub struct Album {
    id: u32,
    name: String,
    year: u32,
    picture: Vec<u8>,
    total_tracks: u32,
    artist_id: u32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Artist)]
#[belongs_to(Album)]
#[table_name = "tracks"]
pub struct Track {
    id: u32,
    title: String,
    year: u32,
    number: u32,
    path: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    artist_id: u32,
    album_id: u32,
}
