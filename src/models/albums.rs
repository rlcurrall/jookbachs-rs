use crate::{models::artists::Artist, schema::albums};

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Artist)]
#[table_name = "albums"]
pub struct Album {
    id: u32,
    name: String,
    year: Option<i32>,
    picture: Option<Vec<u8>>,
    total_tracks: Option<i32>,
    artist_id: i32,
}

#[derive(Insertable)]
#[table_name = "albums"]
pub struct NewAlbum {
    pub name: String,
    pub year: Option<i32>,
    pub picture: Option<Vec<u8>>,
    pub total_tracks: Option<i32>,
    pub artist_id: i32,
}
