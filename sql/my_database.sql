create table users(
	id int auto_increment primary key,
  username varchar(30) not null UNIQUE,
  pwd varchar(255) not null,
  salt varchar(255) not null
);

create table products(
  id int auto_increment primary key,
  name varchar(30) not null,
  description varchar(50) default "",
  stock int default 0
);

create table tags(
  id int auto_increment primary key,
  name varchar(30) not null
);

create table productsTotags(
  productID int,
  tagID int,
  primary key(productID,tagID),
  foreign key (productID) references products(id) on delete cascade on update cascade,
  foreign key (tagID) references tags(id) on delete cascade on update cascade
);