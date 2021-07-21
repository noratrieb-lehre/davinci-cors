# API

* Version 0.1
* Base Route: `/api`

### Event Dao

```json
{
  "id": "uuid",
  "type": "EventType",
  "name": "string",
  "start": "Timestamp",
  "end?": "Timestamp",
  "description": "string"
}
```

### EventType

`"homework" | "exam" | "holidays" | "other"`

### Timestamp

`number`, UTC Unix Timestamp in milliseconds

### Class Dao

```json
{
  "id": "uuid",
  "owner": "User",
  "members": [
    "Member"
  ],
  "name": "string",
  "description": "string"
}
```

### User Dao

```json
{
  "id": "uuid",
  "email": "string",
  "description": "string",
  "classes?": [
    "Class"
  ]
}
```

### Member Dao

```json
{
  "user": "uuid",
  "class": "uuid",
  "displayName": "string",
  "role": "owner | admin | member"
}
```  

### Timetable Dao

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

#### Post myself (not hugo)

`POST /users`           
*Request*           
`User` with password, id optional (will be overriden) and description optional
*Response*           
Refresh-Token: Bearer token              
Token: Bearer token         
`User` & `{"expires": "Timestamp"}`

#### Delete myself (not hugo)

Requires Token
`DELETE /users/me`

### Classes

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
Requires Token    
*Request*    
`Class`  (`members` field ignored)  
*Response*    
`Class`

#### Delete Class

`DELETE /classes/{uuid}`  
Requires Token  
*Response*  
"Deleted class."

### Class member

### Put class member

`PUT /classes/{uuid}/members/{uuid}`  
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
Requires Token  
*Request*

```json
[
  "User"
]
```

#### Accept Member

`POST /classes/{uuid}/requests/{uuid}`  
Requires Token  
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
Requires Token  
*Request*  
`Event` without UUID required  
*Response*  
`Event`

#### Put Event

`POST /classes/{uuid}/events/{uuid}`  
Requires Token  
*Request*  
`Event`  
*Response*  
`Event`

#### Delete Event

`DELETE /classes/{uuid}/events/{uuid}`  
Requires Token

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
Requires Token  
*Response*  
`Timetable`


#### PUT Timetable

`PUT /classes/{uuid}/timetable`  
Requires Token  
*Request*  
`Timetable`  
*Response*  
`Timetable`
