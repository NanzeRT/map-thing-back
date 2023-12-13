create table routes (
    id serial primary key,
    name varchar(255) not null,
    description text,
    author_id int not null references users(id),
    stars int not null default 0
);
