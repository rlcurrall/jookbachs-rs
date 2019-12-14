use crate::schema::artists;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "artists"]
pub struct Artist {
    id: u32,
    name: String,
}

#[derive(Insertable)]
#[table_name = "artists"]
pub struct NewArtist {
    pub name: String,
}
