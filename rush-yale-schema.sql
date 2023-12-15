create table if not exists event
(
    id          int auto_increment
        primary key,
    name        varchar(255) not null,
    description varchar(255) null,
    location    varchar(255) not null,
    start_time  datetime     not null,
    end_time    datetime     not null
);

create table if not exists organization
(
    id    int auto_increment
        primary key,
    name  varchar(255) not null,
    email varchar(255) not null
);

create table if not exists event_organization
(
    id              int auto_increment
        primary key,
    event_id        int not null,
    organization_id int not null,
    constraint event_organization_event_id_fk
        foreign key (event_id) references event (id),
    constraint event_organization_organization_id_fk
        foreign key (organization_id) references organization (id)
);

create table if not exists thread
(
    id int auto_increment
        primary key
);

create table if not exists user
(
    id         int auto_increment
        primary key,
    netid      char(7)      not null,
    first_name varchar(255) not null,
    last_name  varchar(255) not null,
    email      varchar(255) not null,
    location   varchar(255) not null,
    constraint user_pk2
        unique (netid)
);

create table if not exists thread_message
(
    id        int auto_increment
        primary key,
    thread_id int                                 not null,
    sender_id int                                 not null,
    contents  varchar(255)                        not null,
    time      timestamp default CURRENT_TIMESTAMP null,
    constraint thread_message_thread_id_fk
        foreign key (thread_id) references thread (id),
    constraint thread_message_user_id_fk
        foreign key (sender_id) references user (id)
);

create table if not exists thread_reader
(
    id        int auto_increment
        primary key,
    thread_id int not null,
    user_id   int not null,
    constraint thread_reader_thread_id_fk
        foreign key (thread_id) references thread (id),
    constraint thread_reader_user_id_fk
        foreign key (user_id) references user (id)
);

create table if not exists thread_writer
(
    id        int auto_increment
        primary key,
    thread_id int not null,
    user_id   int not null,
    constraint thread_writer_thread_id_fk
        foreign key (thread_id) references thread (id),
    constraint thread_writer_user_id_fk
        foreign key (user_id) references user (id)
);

create table if not exists user_status_option
(
    id   int auto_increment
        primary key,
    name varchar(255) not null
);

create table if not exists event_invitee
(
    id                    int auto_increment
        primary key,
    event_id              int not null,
    user_id               int null,
    organization_id       int null,
    user_status_option_id int null,
    constraint event_invitee_event_id_fk
        foreign key (event_id) references event (id),
    constraint event_invitee_organization_id_fk
        foreign key (organization_id) references organization (id),
    constraint event_invitee_user_id_fk
        foreign key (user_id) references user (id),
    constraint event_invitee_user_status_option_id_fk
        foreign key (user_status_option_id) references user_status_option (id)
);

create table if not exists user_status
(
    user_id               int not null,
    organization_id       int not null,
    user_status_option_id int not null,
    primary key (user_id, organization_id),
    constraint user_status_organization_id_fk
        foreign key (organization_id) references organization (id),
    constraint user_status_user_id_fk
        foreign key (user_id) references user (id),
    constraint user_status_user_status_option_id_fk
        foreign key (user_status_option_id) references user_status_option (id)
);

