CREATE TABLE guilds
(
    id            VARCHAR(20) PRIMARY KEY,
    notif_channel       VARCHAR(20) NULL,
    notif_ping_role     VARCHAR(20) NULL,
    notif_ping_everyone BOOLEAN     NOT NULL DEFAULT FALSE,
    CONSTRAINT guild_id_fk FOREIGN KEY (id)
        REFERENCES classes (discord_id)
        ON DELETE CASCADE
);

INSERT INTO guilds (id)
SELECT discord_id
from classes
WHERE discord_id IS NOT NULL;

ALTER TABLE events
    ADD COLUMN
        notification TIMESTAMP NULl;