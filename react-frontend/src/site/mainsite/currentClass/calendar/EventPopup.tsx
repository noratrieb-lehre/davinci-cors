import React from 'react';
import {Button, Modal} from "react-bootstrap";
import Event from "../../../../data/event/Event";
import EventType from "../../../../data/event/EventType";
import {formatType} from "./Calendar";

const EventPopup = ({event, onClose}: {event: Event, onClose: () => void }) => {
    return (
        <Modal>
            <Modal.Title>{event.name}</Modal.Title>
            <Modal.Body>
                <p>Start: {formatDate(event.start)}</p>
                {
                    event.end && <p>Ende: {formatDate(event.end)}</p>
                }
                <p>Beschreibung: {event.description}</p>
                <p>Typ: {formatType(event.type)}</p>
            </Modal.Body>
            <Modal.Footer><Button onClick={onClose}>Schliessen</Button></Modal.Footer>
        </Modal>
    );
};



const formatDate = (timestamp: number): string => {
    const date = new Date(timestamp);
    const minute = '0' + date.getMinutes();
    const hours = '0' + date.getHours();

    return `${hours.substr(-2)}:${minute.substr(-2)} ${date.getDate()}.${date.getMonth()}.${date.getFullYear()}`
}

export default EventPopup;