create table albums
(
	id integer not null
		constraint albums_pk
			primary key autoincrement,
	name text not null,
	year integer not null,
	picture blob,
	total_tracks integer not null,
	artist_id integer not null
		constraint artist___fk
			references artists
				on delete cascade
);

create unique index albums_id_uindex
	on albums (id);