create table jobs (
  id varchar (36) not null,
  name varchar (32) not null,
  state varchar (32) not null default "created",
  primary key (id)
);
