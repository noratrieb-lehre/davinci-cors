import React from 'react';
import {Col, Container, Row} from "react-bootstrap";
import ClassList from "./currentClass/ClassList";
import ClassView from "./currentClass/ClassView";


const MainSite = () => {
    return (
        <Container fluid>
            <Row>
                <Col sm={2}><ClassList/></Col>
                <Col sm={8}><ClassView/></Col>
            </Row>
        </Container>
    );
};

export default MainSite;