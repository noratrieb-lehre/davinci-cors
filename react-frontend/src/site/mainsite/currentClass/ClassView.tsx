import React from 'react';
import useClassContext from "../useClassContext";
import {Container, Tab, Tabs} from "react-bootstrap";
import ClassInfo from "./ClassInfo";
import Timetable from "./Timetable";
import Events from "./Events";

const ClassView = () => {
    const [currentClass] = useClassContext();
    return (
        <Container>
            {
                currentClass &&  (
                    <Tabs id={'classview-tab'} className={'mb-3'}>
                        <Tab eventKey={'info'} title={'Info'}><ClassInfo/></Tab>
                        <Tab eventKey={"timetable"} title={'Stundeplan'}><Timetable/></Tab>
                        <Tab eventKey={'events'} title={'Events'}><Events/></Tab>
                    </Tabs>
                )
            }
        </Container>
    );
};

export default ClassView;