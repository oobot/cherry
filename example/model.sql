create table if not exists `simple` (
    `id` int unsigned not null,
    `name` varchar(64) not null,
    primary key (`id`)
) engine = InnoDB;