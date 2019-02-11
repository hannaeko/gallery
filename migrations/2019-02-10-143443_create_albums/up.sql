create table albums (
  id varchar (36) not null,
  name varchar (255) not null,
  parent_album_id varchar (36) default null,
  primary key (id),
  foreign key (parent_album_id) references albums(id)
);
