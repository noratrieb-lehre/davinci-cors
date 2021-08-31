import React, {useContext, useEffect, useState} from 'react';
import ModalTitle from 'react-bootstrap/ModalTitle';
import Container from 'react-bootstrap/Container';
import {CurrentClass} from "../ClassView";
import {UserServiceContext} from "../../../Router";
import TimeTable from "../../../../data/timetable/TimeTable";
import TimeTableDay from "./TimeTableDay";

const Timetable = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);
    const [timeTable, setTimeTable] = useState<TimeTable | undefined>();
    useEffect(() => {
        if (currentClass) {
            userService.getTimeTable(currentClass!.id).then(val => {
                setTimeTable(val)
            })
        }
        // eslint-disable-next-line
    }, [currentClass])

    const onLessonDelete = (start: number, end: number, subject: string, idx: number) => {
        if (timeTable) {
            const newDay = timeTable[idx].filter((val) => val.subject !== subject && val.start !== start && val.end !== end)
            userService.updateTimetable(currentClass!.id, newDay, idx)
                .then(() => setTimeTable([...timeTable.filter((val, valIdx) => valIdx !== idx), newDay] as TimeTable))
        }
    }
    return (
        <Container>
            {
                timeTable && timeTable.map((val, idx) => {
                    if (val.length > 0) {
                        return (
                            <TimeTableDay key={idx} name={getNameOfDay(idx)} lessons={val} onLessonDelete={onLessonDelete}/>
                        )
                    } else {
                        return (<Container key={idx}/>)
                    }
                })
            }
            {
                !timeTable && <ModalTitle>Der Stundenplan muss zuerst im Admin Panel erstellt werden</ModalTitle>
            }
        </Container>
    );
};

const getNameOfDay = (idx: number): string =>
    ['Montag', 'Dienstag', 'Mittwoch', 'Donnerstag', 'Freitag', 'Samstag', 'Sonntag'][idx] || ''


export default Timetable;
export {getNameOfDay}