use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};
// use std::fs::{self, DirEntry};
// use std::io;
// use std::path::Path;

// use crate::models::NewArtist;
// use crate::schema::artists;

// let artist = NewArtist {
//     name: String::from("Test"),
// };

// diesel::insert_into(artists::table)
//     .values(&artist)
//     .execute(&pool.get().unwrap())
//     .expect("Error saving artist.diesel");

pub fn database() -> Pool<ConnectionManager<SqliteConnection>> {
    let conn_spec = std::env::var("DB_HOST").expect("DB_HOST");
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    diesel_migrations::run_pending_migrations_in_directory(
        &pool.get().unwrap(),
        std::path::Path::new("migrations"),
        &mut std::io::stdout(),
    )
    .unwrap();

    pool
}
