import React, {useContext, useEffect, useState} from 'react';
import {CurrentClass} from "../ClassView";
import FullCalendar, {EventClickArg, EventContentArg} from "@fullcalendar/react";
import dayGridPlugin from '@fullcalendar/daygrid';
import timeGridPlugin from '@fullcalendar/timegrid'
import interactionPlugin from "@fullcalendar/interaction";
import {UserServiceContext} from "../../../Router";
import Event from "../../../../data/event/Event";
import {Container} from "react-bootstrap";
import EventType from "../../../../data/event/EventType";
import EventPopup from "./EventPopup";
import bootstrapPlugin from '@fullcalendar/bootstrap';

import '@fortawesome/fontawesome-free/css/all.css';

const Calendar = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext)
    const [events, setEvents] = useState<Array<Event>>([]);
    const [selectedEvent, setSelectedEvent] = useState<Event>();
    useEffect(() => {
        userService.getCalendar(currentClass!.id).then(setEvents);
    }, [currentClass, userService])

    const handleEventClick = (event: EventClickArg) => {
        setSelectedEvent({
            name: event.event.title,
            start: event.event.start!.getTime(),
            end: event.event.end?.getTime(),
            description: event.event.extendedProps.description,
            type: event.event.extendedProps.type
        })
    }

    return (
        <Container fluid>
            {
                selectedEvent && (
                    <>
                        <EventPopup event={selectedEvent} onClose={() => setSelectedEvent(undefined)}/>
                    </>
                )
            }
            <Container>
                <FullCalendar
                    plugins={[dayGridPlugin, interactionPlugin, timeGridPlugin, bootstrapPlugin]}
                    headerToolbar={{
                        start: 'prev,next today',
                        center: 'title',
                        right: 'dayGridMonth,timeGridWeek,timeGrid',
                    }}
                    buttonText={{
                        today: 'Heute',
                        month: 'Monat',
                        week: 'Woche',
                        day: 'Tag',
                        timeGrid: 'Tag'
                    }}
                    events={events.map((val) => ({
                        ...val,
                        title: val.name,
                        start: val.start,
                        end: (val.end) ? val.end : undefined,
                        allDay: val.type === 'holidays',
                        backgroundColor: getColorOfEvent(val.type),
                        borderColor: getColorOfEvent(val.type),
                    }))}
                    editable={true}
                    selectable={true}
                    selectMirror={true}
                    eventClick={handleEventClick}
                    eventContent={renderEventContent}
                    firstDay={1}
                    themeSystem={'bootstrap'}
                    locale={'de'}
                    allDayText={'Ganztägig'}
                    allDaySlot={false}
                />
            </Container>
        </Container>
    );
};

const renderEventContent = (eventContent: EventContentArg) => {
    return (
        <>
            <div className="fc-daygrid-event-dot" style={{
                borderColor: eventContent.borderColor
            }}/>
            <div className="fc-event-title">
                {
                    eventContent.event.title.substr(0, 14) + (eventContent.event.title.length > 17 ? '...' : '')
                }
            </div>
        </>
    )
}

const color = {
    'homework': '#b1b3fa',
    'exam': '#eed850',
    'holidays': '#05c61c',
    'other': '#a5a5a5'
}

const getColorOfEvent = (event: EventType) => color[event]

const formatType = (eventType: EventType): string => {
    switch (eventType) {
        case 'homework':
            return 'Hausaufgabe'
        case "exam":
            return 'Prüfung'
        case "holidays":
            return 'Ferien'
        case "other":
            return 'Andere'
    }
}

export default Calendar;
export {formatType}