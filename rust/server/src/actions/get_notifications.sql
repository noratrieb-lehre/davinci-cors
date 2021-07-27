SELECT e.id                  AS "e_id",
       e.class               AS "e_class",
       e.e_type              AS "e_e_type",
       e.name                AS "e_name",
       e.start               AS "e_start",
       e."end"               AS "e_end",
       e.description         AS "e_description",
       e.notification        AS "e_notification",
       g.id                  AS "g_id",
       g.notif_channel       AS "g_notif_channel",
       g.notif_ping_role     AS "g_notif_ping_role",
       g.notif_ping_everyone AS "g_notif_ping_everyone"

FROM events e
         JOIN classes c
              ON e.class = c.id
         JOIN guilds g
              ON c.discord_id = g.id

WHERE notification IS NOT NULL
  AND notification < ? -- current_timestamp
  AND notification > ? -- last timestamp
  AND g.notif_channel IS NOT NULL

-- It's very important for the current_timestamp used to be the exact same timestamp as the one sent back in the response
-- This is achieved by selecting it separately (because it is always required, even if no notification is returned here)
-- and then binding it into this query. If we used the built-in timestamp here, some notifications might get sent multiple times
-- if they are on the edge