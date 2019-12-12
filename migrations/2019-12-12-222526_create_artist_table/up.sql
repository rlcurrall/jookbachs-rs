create table artists
(
	id integer
		constraint artists_pk
			primary key autoincrement,
	name text not null
);

create unique index artists_id_uindex
	on artists (id);

create unique index artists_name_uindex
	on artists (name);