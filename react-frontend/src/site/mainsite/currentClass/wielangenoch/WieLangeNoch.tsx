import React, {useContext, useEffect, useState} from 'react';
import {Container} from "react-bootstrap";
import {CurrentClass} from "../ClassView";
import {UserServiceContext} from "../../../Router";
import {getNameOfDay} from "../timetable/Timetable";
import Lesson from "../../../../data/timetable/Lesson";
import {toLocaleDate} from "../timetable/TimeTableDay";

type WieLangeNochValues = {
    currentDay: string
    currentTime: string,
    currentLesson: string,
    timeTillLessonFinish: string,
    timeTillSchoolFinish: string,
}

type ReturnValue = {
    subject: string, timeTillLessonFinish: string, timeTillSchoolFinish: string
}

const WieLangeNoch = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);
    const [currentInterval, setCurrentInterval] = useState<NodeJS.Timer>();
    const [currentTimeTable, setCurrentTimeTable] = useState<Array<Lesson>>();
    const [value, setValue] = useState<WieLangeNochValues>({
        currentDay: '',
        currentTime: '',
        currentLesson: '',
        timeTillLessonFinish: '',
        timeTillSchoolFinish: '',
    });
    useEffect(() => {
        if (currentInterval)
            clearInterval(currentInterval);
        userService.getTimeTable(currentClass!.id).then(val => {
            setCurrentTimeTable(val[getIndex(new Date())].sort((a, b) => a.end - b.end))
        })
        const interval = setInterval(() => {
            const date = new Date();
            const index = getIndex(date)
            const currentTime = `${formatTime(date.getHours())}:${formatTime(date.getMinutes())}:${formatTime(date.getSeconds())}`
            const currentDay = getNameOfDay(index);
            const currentLesson: ReturnValue = getLessonAndTimes(date)
            setValue({
                currentTime,
                currentDay,
                currentLesson: currentLesson.subject,
                timeTillLessonFinish: currentLesson.timeTillLessonFinish,
                timeTillSchoolFinish: currentLesson.timeTillSchoolFinish
            })
        }, 1000)
        setCurrentInterval(interval);
    }, [currentClass])

    const getLessonAndTimes = (date: Date): ReturnValue => {
        if (currentTimeTable) {
            if (date.getTime() < toLocaleDate(currentTimeTable[0].start).getTime()
                || date.getTime() > toLocaleDate(currentTimeTable[currentTimeTable.length - 1].end).getTime())
                return {
                    subject: 'Keine Schule!',
                    timeTillLessonFinish: '00:00',
                    timeTillSchoolFinish: '00:00:00'
                }
            const currentLesson = currentTimeTable.filter((val) => {
                const start = toLocaleDate(val.start);
                const end = toLocaleDate(val.end);
                return date.getTime() >= start.getTime() && date.getTime() < end.getTime()
            })
            if (currentLesson[0]) {
                return {
                    subject: currentLesson[0].subject,
                    timeTillLessonFinish: getDateDiff(date, currentLesson[0].end),
                    timeTillSchoolFinish: getDateDiff(date, currentLesson[currentLesson.length - 1].end)
                }
            }

            const nextLesson = currentTimeTable.find((val) => date.getTime() < toLocaleDate(val.start).getTime())
            return {
                subject: `Pause (Nächste Lektion: ${nextLesson!.subject})`,
                timeTillLessonFinish: getDateDiff(date, nextLesson!.start),
                timeTillSchoolFinish: getDateDiff(date, currentTimeTable[currentTimeTable.length - 1].end)
            }

        }
        return {
            subject: 'N/A',
            timeTillSchoolFinish: '00:00:00',
            timeTillLessonFinish: '00:00'
        };
    }
    return (
        <Container>
            <p>Heutiger Tag: {value.currentDay}</p>
            <p>Aktuelle Zeit: {value.currentTime}</p>
            <p>Aktuelle Lektion: {value.currentLesson}</p>
            <p>Zeit bis Lektionsende: {value.timeTillLessonFinish}</p>
            <p>Zeit bis Schulende: {value.timeTillSchoolFinish}</p>
        </Container>
    );
};

const getDateDiff = (date1: Date, date2: number): string => {
    const diffTimeSchool = Math.abs(toLocaleDate(date2).getTime() - date1.getTime())
    const hours = Math.floor(diffTimeSchool / 1000 / 60 / 60);
    const minutes = Math.floor(diffTimeSchool / 1000 / 60);
    const seconds = Math.floor(diffTimeSchool / 1000);

    return (hours > 0 ? `${('0' + hours).substr(-2)}:` : '') + `${('0' + minutes).substr(-2)}:${('0' + seconds).substr(-2)}`
}

const formatTime = (number: number): string => {
    return ('0' + number).substr(-2)
}


const getIndex = (date: Date) => (date.getDay() > 0) ? date.getDay() - 1 : 6;


export default WieLangeNoch;