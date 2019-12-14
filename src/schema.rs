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
        year -> Nullable<Integer>,
        picture -> Nullable<Blob>,
        total_tracks -> Nullable<Integer>,
        artist_id -> Integer,
    }
}

table! {
    tracks (id) {
        id -> Integer,
        title -> Text,
        year -> Nullable<Integer>,
        number -> Nullable<Integer>,
        path -> Text,
        album_id -> Integer,
        artist_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
