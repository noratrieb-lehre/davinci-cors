import React, {useContext, useEffect, useState} from 'react';
import {Container} from "react-bootstrap";
import {CurrentClass} from "../ClassView";
import {UserServiceContext} from "../../../Router";
import TimeTable from "../../../../data/timetable/TimeTable";
import TimeTableDay from "./TimeTableDay";

const Timetable = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);
    const [timeTable, setTimeTable] = useState<TimeTable>();
    useEffect(() => {
        if (currentClass) {
            userService.getTimeTable(currentClass!.id).then(val => {
                setTimeTable(val)
            })
        }
    }, [currentClass])
    return (
        <Container>
            {
                timeTable?.map((val, idx) => {
                    if (val.length > 0) {
                        return (
                            <TimeTableDay key={idx} name={getNameOfDay(idx)} lessons={val}/>
                        )
                    } else {
                        return (<Container key={idx}/>)
                    }
                })
            }
        </Container>
    );
};

const getNameOfDay = (idx: number): string =>
    ['Montag', 'Dienstag', 'Mittwoch', 'Donnerstag', 'Freitag', 'Samstag', 'Sonntag'][idx] || ''


export default Timetable;
export {getNameOfDay}