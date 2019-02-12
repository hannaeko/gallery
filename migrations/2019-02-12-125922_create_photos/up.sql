create table photos (
  id varchar (36) not null,
  name varchar (255) not null,
  album_id varchar (36) not null,
  -- metadata
  creation_date datetime,
  flash varchar (255),
  aperture varchar (10),
  exposure_time varchar (10),
  focal_length varchar (10),
  focal_length_in_35mm varchar (10),
  camera varchar (60),
  primary key (id),
  foreign key (album_id) references photos(id)
);
