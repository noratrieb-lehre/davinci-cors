import React from 'react';
import {Button, Col, Container, Form, FormControl, FormGroup, FormLabel, Row} from "react-bootstrap";

const ChangeClassName = () => {
    return (
        <Container>
            <Form inline>
                <Row>
                    <FormLabel>Neuer Klassename</FormLabel>
                </Row>
                <Row>
                    <Col>
                        <FormGroup>
                            <FormControl type={'text'}/>
                        </FormGroup>
                    </Col>
                    <Col>
                        <Button type={'submit'}>Namen ändern</Button>
                    </Col>
                </Row>
            </Form>
        </Container>
    );
};

export default ChangeClassName;