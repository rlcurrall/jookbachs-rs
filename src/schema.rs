table! {
    artists (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    albums (id) {
        id -> Integer,
        name -> Text,
        year -> Integer,
        picture -> Blob,
        total_tracks -> Integer,
        artist_id -> Integer,
    }
}

table! {
    tracks (id) {
        id -> Integer,
        title -> Text,
        year -> Integer,
        number -> Integer,
        path -> Text,
        album_id -> Integer,
        artist_id -> Integer,
        created_at -> Date,
        updated_at -> Date,
    }
}
