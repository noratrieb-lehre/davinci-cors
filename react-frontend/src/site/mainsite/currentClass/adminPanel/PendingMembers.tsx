import React, {useContext, useEffect, useState} from 'react';
import {Button, Col, Container, ListGroup, ModalTitle, Row} from "react-bootstrap";
import {CurrentClass} from "../ClassView";
import User from "../../../../data/user/User";
import {UserServiceContext} from "../../../Router";
import * as Icon from 'react-bootstrap-icons';

const PendingMembers = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);
    const [pendingMembers, setPendingMembers] = useState<Array<User>>();

    useEffect(() => {
        userService.getPendingMembers(currentClass!.id).then(setPendingMembers)
    }, [])

    return (
        <Container>

            {
                pendingMembers && pendingMembers.length > 0 && <>
                    <hr/>
                    <ModalTitle>Anfragen von Benutzer</ModalTitle>
                </>
            }
            <ListGroup>
                {
                    pendingMembers?.map(val =>
                        <ListGroup.Item>
                            <Row>
                                <Col>{val.email}</Col>
                                <Col><Button variant={'outline-success'}><Icon.Check color={'green'}/></Button></Col>
                                <Col><Button variant={'outline-danger'}><Icon.X color={'red'}/></Button></Col>
                            </Row>
                        </ListGroup.Item>
                    )
                }
            </ListGroup>
        </Container>
    );
};

export default PendingMembers;