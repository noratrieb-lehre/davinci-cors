# API

* Version 0.1
* Base Route: `/api`

## Dtos
<field>? -> Optional for requests, always present in responses
<field>?? -> Optional for requests, optional responses

### Event Dto

```json
{
  "id?": "uuid",
  "type": "EventType",
  "name": "string",
  "start": "Timestamp",
  "end?": "Timestamp | null",
  "description": "string"
}
```

### EventType

`"homework" | "exam" | "holidays" | "other"`

### Timestamp

`number`, UTC Unix Timestamp in milliseconds
  
### SnowflakeId

`string`, Discord Snwoflake Id

### Class Dto

```json
{
  "id?": "uuid",
  "owner": "User",
  "members??": [
    "Member"
  ],
  "name": "string",
  "description": "string"
}
```

### User Dto

```json
{
  "id?": "uuid",
  "email": "string",
  "description?": "string",
  "classes??": [
    "Class"
  ]
}
```

### Member Dto

```json
{
  "user?": "uuid",
  "displayName": "string",
  "role": "owner | admin | member"
}
```  

### Timetable Dto

```json
[
  "TimetableDay",
  "TimetableDay",
  "TimetableDay",
  "TimetableDay",
  "TimetableDay",
  "TimetableDay",
  "TimetableDay"
]
```

### TimetableDay

```json
[
  "Lesson"
]
```

### Lesson

```json
{
  "subject": "string",
  "description": "string",
  "start": "DayTimestamp",
  "end": "DayTimestamp"
}
```

### DayTimestamp

`number`, UTC, milliseconds since 00:00

## Routes

### Hugo

`GET /hugo`    
*Response*

```json  
"Hugo Boss"  
```  

### Auth

#### Login

`POST /login`

*Request*

```json  
{
  "email": "string",
  "password": "string"
}
```

*Response*

Refresh-Token: Bearer token   
Token: Bearer token

```json
{
  "userid": "uuid",
  "expires": "Timestamp"
}
```

#### Token

`GET /token`

*Request*

Authorization: Bearer token

*Response*
Token: Bearer token

```json
{
  "expires": "Timestamp"
}
```

### Users

#### Get myself (not hugo)

`GET /users/me`  
Requires Token  
*Response*  
`User`

#### Put myself (not hugo)

`PUT users/me`  
Requires Token  
*Request*  
`User`  
*Response*  
`User`
  
#### Change password

`PATCH users/me/password`  
Requires Token  
*Request*  
```json
{
  "password": "string",
  "oldPassword": "string"
}```  
*Response*  
`User`

#### Post myself (not hugo)

`POST /users`           
*Request*           
`User` with password
*Response*           
Refresh-Token: Bearer token              
Token: Bearer token         
`{"expires": "Timestamp", "user": "User"}`

#### Delete myself (not hugo)

Requires Token
`DELETE /users/me`

### Classes

Getting information about a class requires being in that class

#### Get class

`GET /classes/{uuid}`  
Requires Token  
*Response*  
`Class`

#### Post class

`POST /classes`  
Requires Token  
*Request*

```json
{
  "name": "string",
  "description": "string"
}
```

*Response*
`Class`

#### Put class

`PUT /classes/{uuid}`   
Requires Token & Admin     
*Request*    
`Class`  (`members` field ignored)  
*Response*    
`Class`

#### Delete Class

`DELETE /classes/{uuid}`  
Requires Token & Owner  
*Response*  
"Deleted class."

### Class member

### Put class member
Requires Token & Admin  
`PUT /classes/{uuid}/members/{uuid}`  
*Request*  
`Member`    
*Response*  
`Member`

### Delete class member
Requires Token & Admin  
`DELETE /classes/{uuid}/members/{uuid}`  
*Request*  
`Member`    
*Response*  
`Member`


#### Request join

`POST /classes/{uuid}/join`    
Requires Token    
*Response*    
"Pending approval..."

#### See join request users

`GET /classes/{uuid}/requests`  
Requires Token & Admin  
*Request*

```json
[
  "User"
]
```

#### Accept Member

`POST /classes/{uuid}/requests/{uuid}`  
Requires Token & Admin  
*Request*

```json
{
  "accept": "boolean"
}
```

### Events

#### Get Event

`GET /classes/{uuid}/events/{uuid}`    
Requires Token    
*Response*  
`Event`

#### Get Events

`GET /classes/{uuid}/events?before=Timestamp&after=Timestamp`  
Requires Token  
Parameters not required  
*Response*

```json
[
  "Event"
]
```

#### Post Event

`POST /classes/{uuid}/events`  
Requires Token & Admin  
*Request*  
`Event` without UUID required  
*Response*  
`Event`

#### Put Event

`POST /classes/{uuid}/events/{uuid}`  
Requires Token & Admin  
*Request*  
`Event`  
*Response*  
`Event`

#### Delete Event

`DELETE /classes/{uuid}/events/{uuid}`  
Requires Token & Admin

### Timetable

#### GET Timetable

`GET /classes/{uuid}/timetable`  
Requires Token  
*Request*  
`Timetable`  
*Response*  
`Timetable`

#### POST Timetable

`POST /classes/{uuid}/timetable`  
Requires Token & Admin  
*Response*  
`Timetable`


#### PUT Timetable

`PUT /classes/{uuid}/timetable`  
Requires Token & Admin  
*Request*  
`Timetable`  
*Response*  
`Timetable`


## Discord routes

### Link user with discord user
Requires token  
`POST /users/me/link`
*Request*
```json
{
  "snowflake": "SnowflakeId"
}
```

### Link guild with class
Requires token & Owner  
`POST /classes/{uuid}/link`
*Request*
```json
{
  "snowflake": "SnowflakeId"
}
```

#### Get class with discord snowflake

`GET /classes/discord/{snowflake}`  
Bot only 
*Response*  
`Class`

#### Get user with discord snowflake

`GET /users/discord/{snowflake}`  
Bot only  
*Response*  
`User`

#### Notifications
##### Notification
```json
{
  "event": "Event",
  "guild": "SnowflakeId",
  "channel": "SnowflakeId",
  "rolePing": "SnowflakeId | null",
  "everyonePing": "boolean"
}
```  
  
`/bot/notifications?since=lastTimestamp`  
Bot only  

Get all events + notification data for events that had their notifications due in the time since the last timestamp.  
*Response*  
`{"notifications": "Notification[]", "time": "Timestamp"}`
