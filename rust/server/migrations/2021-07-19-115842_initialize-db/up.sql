CREATE EXTENSION pgcrypto;

CREATE TABLE users
(
    id          UUID PRIMARY KEY,
    email       VARCHAR(50),
    password    TEXT,
    description VARCHAR(1000)
);

CREATE TABLE member_role
(
    id      INT PRIMARY KEY,
    display VARCHAR(10) NOT NULL
);

CREATE TABLE class
(
    id          UUID PRIMARY KEY,
    owner       UUID NOT NULL,
    name        VARCHAR(50) NOT NULL,
    description VARCHAR(50) NOT NULL DEFAULT '',
    timetable   TEXT NOT NULL DEFAULT '[[], [], [], [], [], [], []]', -- no reason to have like 100 joins for one single timetable, something like json fits best
    CONSTRAINT classOwnerFK
        FOREIGN KEY (owner)
            REFERENCES users (id)
);

CREATE TABLE member
(
    "user"       UUID,
    class        UUID,
    display_name VARCHAR(50) NOT NULL,
    role         INT NOT NULL DEFAULT 2,
    PRIMARY KEY ("user", class),
    CONSTRAINT member_users_fk
        FOREIGN KEY ("user")
            REFERENCES users (id),
    CONSTRAINT member_class_fk
        FOREIGN KEY (class)
            REFERENCES class (id),
    CONSTRAINT member_role_fk
        FOREIGN KEY (role)
            REFERENCES member_role (id)
);

INSERT INTO member_role (id, display) VALUES
(0, 'owner'),
(1, 'admin'),
(2, 'member')