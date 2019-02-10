create table albums (
  id int not null,
  name varchar (255) not null,
  parent_ablum_id int,
  primary key (id),
  foreign key (parent_ablum_id) references albums(id)
);
