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
  "description": "string",
  "notification?": "Timestamp | null"
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
  "description": "string",
  "discordId?": "string | null"
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

## Errors
On every Route:        
`400 invalid-uuid`  on a request with an invalid uuid (cannot be parsed to a uuid)    
`401 token-expired`  on a request with an expired token    
`401 no-access`  on a request to a guild that the user is not part of    
`500 Internal Server Error` all the time    
Routes that require admin        
`401 no-admin`  on a request where the user is not admin in that class    
Routes that require owner        
`401 no-owner`  on a request where the user is not the owner in that class    
    
Routes that insert/edit something  
`409 already-exists` (on Unique Violation)  
`409 does-not-exist` (on Foreign Key Violation)  

Others: See routes

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
}
```  
*Response*  
`User`  

Error:
`401 wrong-passord`

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

Errors:  
`401 no-owner` on not being owner  

### Class member
  
### Get class member
Requires Token  
`GET /classes/{uuid}/members/{uuid}`     
*Response*  
`Member`

### Put class member
Requires Token & Admin  
`PUT /classes/{uuid}/members/{uuid}`  
*Request*  
`Member`    
*Response*  
`Member`

Errors:  
`401 edit-own-permission` on editing the own permissions  
`401 not-enough-permissions` on editing a member with a role higher/equal role to own  
`401 not-enough-permissions` on editing a member to have a higher/equal role than the own  

### Delete class member
Requires Token & Admin  
`DELETE /classes/{uuid}/members/{uuid}`  

Errors:  
`400 must-have-owner` on deleting the owner  
`401 not-enough-permissions` on deleting a member with a role higher/equal role to own  


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
["Member"]
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

Errors:
`400 member-not-pending` on accepting a member that is not pending  

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

### Notifications
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

##### Guild Dto

```json
{
  "id": "Snowflake",
  "notifChannel?": "Snowflake | null",
  "notifPingRole?": "Snowflake | null",
  "notifPingEveryone?": "boolean"
}
```
#### Get notifications
`GET /bot/notifications?since=lastTimestamp`  
Bot only  

Get all events + notification data for events that had their notifications due in the time since the last timestamp.  
*Response*  
`{"notifications": "Notification[]", "time": "Timestamp"}`

#### Get Guild
`Get /bot/guilds/{{snowflake}}`  
Bot only  

*Response*  
`Guild`
  
#### Put Guild
`PUT /bot/guilds`  
Bot only  

Change guild settings  
*Request*  
`Guild`  
*Response*  
`Guild`
