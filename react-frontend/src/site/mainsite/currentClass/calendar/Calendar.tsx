import React, {useContext, useEffect, useState} from 'react';
import {CurrentClass} from "../ClassView";
import FullCalendar, {EventClickArg, EventContentArg, EventInput} from "@fullcalendar/react";
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
        setEvents(userService.getCalendar(currentClass!.id))

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
                selectedEvent && <EventPopup event={selectedEvent} onClose={() => setSelectedEvent(undefined)}/>
            }
            <Container>
                <FullCalendar
                    plugins={[dayGridPlugin, interactionPlugin, timeGridPlugin, bootstrapPlugin]}
                    headerToolbar={{
                        start: 'prev,next today',
                        center: 'title',
                        right: 'dayGridMonth,timeGrid'
                    }}
                    events={[...events.map((val): EventInput => ({
                        title: val.name,
                        ...val
                    }))]}
                    editable={true}
                    selectable={true}
                    selectMirror={true}
                    eventClick={handleEventClick}
                    eventContent={renderEventContext}
                    firstDay={1}
                    themeSystem={'bootstrap'}
                    locale={'de'}
                />
            </Container>
        </Container>
    );
};

const renderEventContext = (eventContent: EventContentArg) => {
    return (
        <>
            <span>{eventContent.event.title}</span>
            <span>| {formatType(eventContent.event.extendedProps.type)}</span>
        </>
    )
}

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