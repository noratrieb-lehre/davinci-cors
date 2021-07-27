import React from 'react';
import {Col, Container, Row} from "react-bootstrap";
import ClassList from "./currentClass/ClassList";
import ClassView from "./currentClass/ClassView";
import CreateClass from "../createclass/CreateClass";


const MainSite = () => {
    return (
        <Container fluid>
            <Row>
                <Col md={{span: 2, offset: 10}}>
                    <CreateClass/>
                </Col>
            </Row>
            <Row>
                <Col sm={2}><ClassList/></Col>
                <Col sm={8}><ClassView/></Col>
            </Row>
        </Container>
    );
};

export default MainSite;