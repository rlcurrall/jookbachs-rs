use crate::{
    models::{albums::Album, artists::Artist},
    schema::tracks,
};
use chrono::{offset::Utc, DateTime, NaiveDateTime};

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Artist)]
#[belongs_to(Album)]
#[table_name = "tracks"]
pub struct Track {
    id: i32,
    title: String,
    year: Option<i32>,
    number: Option<i32>,
    path: String,
    album_id: i32,
    artist_id: i32,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[table_name = "tracks"]
pub struct NewTrack {
    pub title: String,
    pub year: Option<i32>,
    pub number: Option<i32>,
    pub path: String,
    pub album_id: i32,
    pub artist_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
