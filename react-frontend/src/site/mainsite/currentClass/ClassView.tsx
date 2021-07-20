import React, {useContext, useEffect, useState} from 'react';
import {Container, Tab, Tabs} from "react-bootstrap";
import {useParams} from 'react-router-dom';
import ClassInfo from "./ClassInfo";
import Timetable from "./Timetable";
import Events from "./Events";
import {UserServiceContext} from "../../Router";
import Class from "../../../data/class/Class";

const CurrentClass = React.createContext<Class | undefined>(undefined)

const ClassView = () => {
    const {id} = useParams<{ id: string }>();
    const [currentClass, setCurrentClass] = useState<Class>();
    const userService = useContext(UserServiceContext);
    useEffect(() => {
        console.log(id)
        if (id) {
            setCurrentClass(userService.getClass(id));
        }
    }, [id])
    return (
        <Container>
            {
                currentClass && (
                    <CurrentClass.Provider value={currentClass}>
                        <Tabs id={'classview-tab'} className={'mb-3'}>
                            <Tab eventKey={'info'} title={'Info'}><ClassInfo/></Tab>
                            <Tab eventKey={"timetable"} title={'Stundenplan'}><Timetable/></Tab>
                            <Tab eventKey={'calendar'} title={'Kalender'}><Events/></Tab>
                        </Tabs>
                    </CurrentClass.Provider>
                )
            }
        </Container>
    );
};


export default ClassView;
export {CurrentClass}