import React, {useContext, useEffect, useState} from 'react';
import {Button, Col, Container, ListGroup, ModalTitle, Row} from "react-bootstrap";
import {CurrentClass} from "../ClassView";
import {UserServiceContext} from "../../../Router";
import * as Icon from 'react-bootstrap-icons';
import Member from "../../../../data/user/Member";

const PendingMembers = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);
    const [pendingMembers, setPendingMembers] = useState<Array<Member>>([]);

    const sendApprovalOrDenial = (bool: boolean, userId: string) => {
        userService.replyToRequest(currentClass!.id, userId, bool).then(() => {
            setPendingMembers((arr) => arr.filter(val => val.user !== userId))
        });
    }

    useEffect(() => {
        userService.getPendingMembers(currentClass!.id).then(setPendingMembers)
        // eslint-disable-next-line
    }, [])

    return (
        <Container>
            <ModalTitle>Anfragen von Benutzer</ModalTitle>
            <br/>
            <ListGroup>
                {
                    pendingMembers?.map(val =>
                        <ListGroup.Item key={val.user}>
                            <Row>
                                <Col sm={8} className={'d-flex align-items-center'}>{val.displayName}</Col>
                                <Col sm={2}><Button variant={'outline-success'}
                                                    onClick={() => sendApprovalOrDenial(true, val.user)}><Icon.Check
                                    color={'green'} size={30}/></Button></Col>
                                <Col sm={2}><Button variant={'outline-danger'}
                                                    onClick={() => sendApprovalOrDenial(false, val.user)}><Icon.X
                                    color={'red'} size={30}/></Button></Col>
                            </Row>
                        </ListGroup.Item>
                    )
                }
            </ListGroup>
            <br/>
        </Container>
    );
};

export default PendingMembers;