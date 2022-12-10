drop table if exists user;
create table user (
    `id` int unsigned not null,
    `name` text not null,
    primary key (`id`)
) without rowid;