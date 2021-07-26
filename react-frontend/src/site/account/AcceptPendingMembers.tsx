import React, {useState} from 'react';
import {Button, Col, Container, ListGroup, Row} from "react-bootstrap";
import User from "../../data/user/User";
import * as Icon from 'react-bootstrap-icons';

const AcceptPendingMembers = () => {
    const [pendingMembers, setPendingMembers] = useState<Array<User>>([]);
    return (
        <Container>
            <ListGroup>
                {
                    pendingMembers.map(val => (<ListGroup.Item>
                        <Row>
                            <Col sm={8}>

                            </Col>
                            <Col sm={8}>
                                <Row>
                                    <Col>
                                        <Button variant={'outline-primary'}><Icon.Check color={'green'}/></Button>
                                    </Col>
                                    <Col>
                                        <Button variant={'outline-primary'}><Icon.X color={'red'}/></Button>
                                    </Col>
                                </Row>
                            </Col>
                        </Row>
                    </ListGroup.Item>))
                }
            </ListGroup>
        </Container>
    );
};

export default AcceptPendingMembers;