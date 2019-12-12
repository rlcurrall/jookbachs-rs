create table tracks
(
	id integer not null
		constraint tracks_pk
			primary key autoincrement,
	path text not null,
	title text not null,
	year integer,
	number integer,
	created_at text not null,
	updated_at text,
	artist_id integer not null
		constraint artists___fk
			references artists
				on delete cascade,
	album_id integer not null
		constraint albums___fk
			references albums
				on delete cascade
);

create unique index tracks_id_uindex
	on tracks (id);

create unique index tracks_path_uindex
	on tracks (path);