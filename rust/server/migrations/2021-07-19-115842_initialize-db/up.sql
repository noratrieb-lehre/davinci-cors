CREATE EXTENSION pgcrypto;

CREATE TABLE users
(
    id          UUID PRIMARY KEY,
    name        VARCHAR(50),
    password    TEXT,
    description VARCHAR(1000)
);

CREATE TABLE member_role
(
    id      INT PRIMARY KEY,
    display VARCHAR(10)
);

CREATE TABLE class
(
    id          UUID PRIMARY KEY,
    owner       UUID,
    name        VARCHAR(50),
    description VARCHAR(50),
    timetable   JSON, -- no reason to have like 100 joins for one single timetable, something like json fits best
    CONSTRAINT classOwnerFK
        FOREIGN KEY (owner)
            REFERENCES users (id)
);

CREATE TABLE member
(
    users         UUID,
    class        UUID,
    display_name VARCHAR(50),
    role         INT,
    PRIMARY KEY (users, class),
    CONSTRAINT member_users_fk
        FOREIGN KEY (users)
            REFERENCES users (id),
    CONSTRAINT member_class_fk
        FOREIGN KEY (class)
            REFERENCES class (id),
    CONSTRAINT member_role_fk
        FOREIGN KEY (role)
            REFERENCES member_role (id)
);