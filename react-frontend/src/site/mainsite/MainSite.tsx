import React, {useEffect, useState} from 'react';
import {Col, Container, Row} from "react-bootstrap";
import ClassList from "./ClassList";
import ClassView from "./currentClass/ClassView";
import Class from "../../data/class/Class";

const ClassContext = React.createContext<[Class | undefined, React.Dispatch<Class | undefined>] | null>(null);

const MainSite = () => {
    const [currentClass, setCurrentClass] = useState<Class>();
    useEffect(() => console.log(currentClass), [currentClass])
    return (
        <Container fluid>
            <ClassContext.Provider value={[currentClass, setCurrentClass]}>
                <Row>
                    <Col sm={2}><ClassList/></Col>
                    <Col sm={8}><ClassView/></Col>
                </Row>
            </ClassContext.Provider>
        </Container>
    );
};

export default MainSite;
export {ClassContext};