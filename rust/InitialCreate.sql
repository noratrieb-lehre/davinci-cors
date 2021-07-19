-- postgresPWD=hugo48hugo

CREATE TABLE user
(
    id          UUID PRIMARY KEY,
    name        VARCHAR(50),
    password    someencryptedthingorsomething,
    description VARCHAR(1000)
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
            REFERENCES user (id)
);

CREATE TABLE member
(
    user         UUID,
    class        UUID,
    display_name VARCHAR(50),
    role         INT,
    PRIMARY KEY (user, class),
    CONSTRAINT member_user_fk
        FOREIGN KEY (user)
            REFERENCES user (id),
    CONSTRAINT member_class_fk
        FOREIGN KEY (class)
            REFERENCES class (id),
    CONSTRAINT member_role_fk
        FOREIGN KEY (role)
            REFERENCES member_role (id)
);

CREATE TABLE member_role
(
    id      INT PRIMARY KEY,
    display VARCHAR(10)
);


-- down
DROP TABLE user;
DROP TABLE class;
DROP TABLE member;
DROP TABLE member_role;
