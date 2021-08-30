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
        userService.getTimeTable(currentClass!.id).then(val => {
            if (val) {
                setCurrentTimeTable(val[getIndex(new Date())].sort((a, b) => a.end - b.end))
            }
        })
        // eslint-disable-next-line
    }, [currentClass])

    useEffect(() => {
        return () => {
            if (currentInterval)
                clearInterval(currentInterval);
        }
    }, [currentInterval])

    useEffect(() => {
        if (currentTimeTable) {
            const timetable = currentTimeTable.map(val => ({
                ...val,
                start: toLocaleDate(val.start),
                end: toLocaleDate(val.end)
            }))

            const interval = setInterval(() => {
                const date = new Date();
                const index = getIndex(date)
                const currentTime = `${formatTime(date.getHours())}:${formatTime(date.getMinutes())}:${formatTime(date.getSeconds())}`
                const currentDay = getNameOfDay(index);
                const currentLesson: ReturnValue = getLessonAndTimes(date, timetable)
                setValue({
                    currentTime,
                    currentDay,
                    currentLesson: currentLesson.subject,
                    timeTillLessonFinish: currentLesson.timeTillLessonFinish,
                    timeTillSchoolFinish: currentLesson.timeTillSchoolFinish
                })
            }, 1000)
            setCurrentInterval(interval);
        }
        // eslint-disable-next-line
    }, [currentTimeTable])


    const getLessonAndTimes = (date: Date, currentTimeTable: Array<{ start: Date, end: Date, subject: string }>): ReturnValue => {
        if (currentTimeTable) {
            if (currentTimeTable.length > 0) {
                if (date.getTime() < currentTimeTable![0]!.start.getTime() ||
                    date.getTime() > currentTimeTable![currentTimeTable!.length - 1].end.getTime())
                return {
                    subject: 'Keine Schule!',
                    timeTillLessonFinish: '00:00',
                    timeTillSchoolFinish: '00:00:00'
                }
                const currentLesson = currentTimeTable!.filter((val) => {
                    const start = val.start;
                    const end = val.end;
                    return date.getTime() >= start.getTime() && date.getTime() < end.getTime()
                })
                if (currentLesson[0]) {
                    return {
                        subject: currentLesson[0].subject,
                        timeTillLessonFinish: getDateDiff(date, currentLesson[0].end.getTime()),
                        timeTillSchoolFinish: getDateDiff(date, currentTimeTable![currentTimeTable!.length - 1].end.getTime())
                    }
                }

                const nextLesson = currentTimeTable!.find((val) => date.getTime() < val.start.getTime())
                if (nextLesson) {
                    return {
                        subject: `Pause (Nächste Lektion: ${nextLesson!.subject})`,
                        timeTillLessonFinish: getDateDiff(date, nextLesson!.start.getTime()),
                        timeTillSchoolFinish: getDateDiff(date, currentTimeTable![currentTimeTable!.length - 1].end.getTime())
                    }
                }
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
    let date = new Date();
    date.setHours(0, 0, 0)
    date.setMilliseconds(date2 - date1.getTime())
    const hours = date.getHours()
    const minutes = date.getMinutes()
    const seconds = date.getSeconds()

    return (hours > 0 ? `${('0' + hours).substr(-2)}:` : '') + `${('0' + minutes).substr(-2)}:${('0' + seconds).substr(-2)}`
}

const formatTime = (number: number): string => {
    return ('0' + number).substr(-2)
}


const getIndex = (date: Date) => (date.getDay() > 0) ? date.getDay() - 1 : 6;


export default WieLangeNoch;