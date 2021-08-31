import React from 'react';
import Container from 'react-bootstrap/Container';
import Col from 'react-bootstrap/Col';
import Row from 'react-bootstrap/Row';
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
                <Col sm={10}><ClassView/></Col>
            </Row>
        </Container>
    );
};

export default MainSite;