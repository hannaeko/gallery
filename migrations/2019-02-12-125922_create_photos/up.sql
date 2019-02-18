create table photos (
  id varchar (36) not null,
  name varchar (255) not null,
  album_id varchar (36) not null,
  hash varchar (256) not null,
  -- metadata
  creation_date datetime,
  camera varchar (60),
  exposure_time varchar (10),
  aperture varchar (10),
  focal_length varchar (10),
  focal_length_in_35mm varchar (10),
  flash varchar (255),
  primary key (id),
  foreign key (album_id) references photos(id)
);
