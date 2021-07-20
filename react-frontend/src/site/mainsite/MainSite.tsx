import React, {useEffect, useState} from 'react';
import {Col, Container, Row} from "react-bootstrap";
import ClassList from "./ClassList";
import ClassView from "./currentClass/ClassView";

const ClassContext = React.createContext<[string | undefined, React.Dispatch<string | undefined>] | null>(null);

const MainSite = () => {
    const [currentClass, setCurrentClass] = useState<string>();
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