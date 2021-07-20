CREATE EXTENSION pgcrypto;

CREATE TABLE users
(
    id          UUID PRIMARY KEY,
    email       VARCHAR(50)   NOT NULL,
    password    TEXT          NOT NULL,
    description VARCHAR(1000) NOT NULL DEFAULT ''
);

CREATE TABLE member_roles
(
    id      INT PRIMARY KEY,
    display VARCHAR(10) NOT NULL
);

CREATE TABLE classes
(
    id          UUID PRIMARY KEY,
    owner       UUID        NOT NULL,
    name        VARCHAR(50) NOT NULL,
    description VARCHAR(50) NOT NULL DEFAULT '',
    timetable   TEXT        NOT NULL DEFAULT '[[], [], [], [], [], [], []]', -- no reason to have like 100 joins for one single timetable, something like json fits best
    CONSTRAINT classOwnerFK
        FOREIGN KEY (owner)
            REFERENCES users (id)
);

INSERT INTO member_roles (id, display)
VALUES (0, 'owner'),
       (1, 'admin'),
       (2, 'member');


CREATE TABLE members
(
    "user"       UUID        NOT NULL,
    class        UUID        NOT NULL,
    display_name VARCHAR(50) NOT NULL,
    role         INT         NOT NULL DEFAULT 2,
    PRIMARY KEY ("user", class),
    CONSTRAINT member_users_fk
        FOREIGN KEY ("user")
            REFERENCES users (id),
    CONSTRAINT member_class_fk
        FOREIGN KEY (class)
            REFERENCES classes (id),
    CONSTRAINT member_role_fk
        FOREIGN KEY (role)
            REFERENCES member_roles (id)
);

CREATE TABLE event_types
(
    id      SERIAL PRIMARY KEY,
    display VARCHAR(10) NOT NULL
);

INSERT INTO event_types (display)
VALUES ('homework'),
       ('exam'),
       ('holidays'),
       ('other');

CREATE TABLE events
(
    id          UUID PRIMARY KEY,
    class       UUID          NOT NULL,
    e_type      INT           NOT NULL DEFAULT 4,
    name        VARCHAR(50)   NOT NULL,
    start       TIMESTAMP     NOT NULL,
    "end"       TIMESTAMP     NULL,
    description VARCHAR(1000) NOT NULL,
    CONSTRAINT event_class_fk
        FOREIGN KEY (class)
            REFERENCES classes (id),
    CONSTRAINT event_type_fk
        FOREIGN KEY (e_type)
            REFERENCES event_types (id)
);