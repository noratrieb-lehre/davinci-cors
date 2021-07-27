# Actix-web backend

### .env File
`DATABASE_URL=postgres://postgres:hugo58hugo@localhost/davinci`  
`JWT_SECRET={{some_secret}}`  
`RUST_LOG=info`


## Notifications
`/bot/notifications?since=lastTimestamp`

Get all events + notification data for events that had their notifications due in the time since the last timestamp.
It works by using a big SQL statement.