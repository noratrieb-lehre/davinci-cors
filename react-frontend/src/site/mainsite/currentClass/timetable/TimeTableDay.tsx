import React, {useContext} from 'react';
import Lesson from "../../../../data/timetable/Lesson";
import Table from 'react-bootstrap/Table';
import Button from 'react-bootstrap/Button';
import ModalTitle from 'react-bootstrap/ModalTitle';
import Container from 'react-bootstrap/Container';
import {CurrentClass} from "../ClassView";
import {UserServiceContext} from "../../../Router";

type Props = { idx: number, name: string, lessons: Array<Lesson>, onLessonDelete: (start: number, end: number, subject: string, idx: number) => void }

const TimeTableDay = ({name, lessons, onLessonDelete, idx}: Props) => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);

    return (
        <Container>
            <ModalTitle>{name}</ModalTitle>
            <Table responsive={'xl'}>
                <thead>
                <tr>
                    <th>Von - Bis</th>
                    <th>Lektion</th>
                    <th>Beschreibung</th>
                    {
                        userService.isAdmin(currentClass!) && <th/>

                    }
                </tr>
                </thead>
                <tbody>
                {
                    lessons.map((val, index) => {
                        return (
                            <tr key={index}>
                                <td>{formatTime(val.start)}-{formatTime(val.end)}</td>
                                <td colSpan={1}>{val.subject}</td>
                                <td colSpan={2}>{val.description}</td>
                                {
                                    userService.isAdmin(currentClass!) && <td colSpan={0.5}>
                                        <Button
                                            onClick={() => onLessonDelete(lessons[index].start, lessons[index].end, lessons[index].subject, idx)}
                                            variant={'outline-danger'}>Lektion löschen</Button>
                                    </td>

                                }
                            </tr>
                        )
                    })
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