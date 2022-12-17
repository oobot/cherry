drop table if exists user;
create table user (
    `id` int unsigned not null,
    `name` text not null,
    `age` int unsigned not null,
    primary key (`id`)
) without rowid;

drop table if exists book;
create table book (
    `id` int unsigned not null,
    `name` text not null,
    `authors` json not null,
    `edition` int unsigned not null,
    `published_date` text not null,
    primary key (`id`)
) without rowid;