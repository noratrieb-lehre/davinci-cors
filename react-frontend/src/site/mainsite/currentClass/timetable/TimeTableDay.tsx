import React from 'react';
import Lesson from "../../../../data/timetable/Lesson";
import {Container, ModalTitle, Table} from "react-bootstrap";

const TimeTableDay = ({name, lessons}: { name: string, lessons: Array<Lesson> }) => {
    return (
        <Container>
            <ModalTitle>{name}</ModalTitle>
            <Table>
                <thead>
                <tr>
                    <th>Von - Bis</th>
                    <th>Lektion</th>
                    <th>Beschreibung</th>
                </tr>
                </thead>
                <tbody>
                {
                    lessons.map((val, idx) => (
                        <tr key={idx}>
                            <td>{formatTime(val.start)}-{formatTime(val.end)}</td>
                            <td>{val.subject}</td>
                            <td>{val.description}</td>
                        </tr>
                    ))
                }
                </tbody>
            </Table>
        </Container>
    );
};

const formatTime = (time: number): string => {
    const date = toLocaleDate(time);
    const minutes = '0' + date.getMinutes();
    const hours = '0' + date.getHours();
    return `${hours.substr(-2)}:${minutes.substr(-2)}`
}

const toLocaleDate = (time: number): Date => {
    const date = new Date();
    date.setHours(0, 0, 0);
    date.setMilliseconds(time)
    return date;
}

export default TimeTableDay;
export {toLocaleDate}