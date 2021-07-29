import React, {useContext, useEffect, useState} from 'react';
import {Button, Col, Container, ListGroup, Row} from "react-bootstrap";
import * as Icon from 'react-bootstrap-icons';
import Member from "../../data/user/Member";
import {UserServiceContext} from "../Router";
import {CurrentClass} from "../mainsite/currentClass/ClassView";
import {useHistory} from "react-router-dom";

const AcceptPendingMembers = () => {
    const userService = useContext(UserServiceContext);
    const currentClass = useContext(CurrentClass);
    const [pendingMembers, setPendingMembers] = useState<Array<Member>>([]);
    const history = useHistory();

    const fetchPendingMembers = () => {
        userService.getPendingMembers(currentClass!.id).then(setPendingMembers).catch((err) => {
            switch (err.message) {
                case 'invalid-uuid':
                    alert('Die ID der Klasse ist nicht gültig.');
                    history.push('/class')
                    break;
                case 'token-expired':
                    userService.forceUpdate().then(() => fetchPendingMembers());
                    break;
                case 'no-admin':
                    window.location.reload();
            }
        })
    }

    useEffect(fetchPendingMembers, [])

    const replyToUser = (userId: string, bool: boolean) => {
        userService.replyToRequest(currentClass!.id, userId, bool).then(() =>
            setPendingMembers(pendingMembers.filter(val => val.user !== userId))
        )
    }

    return (
        <Container>
            <ListGroup>
                {
                    pendingMembers.map(val => (<ListGroup.Item>
                                <Row>
                                    <Col sm={8} className={'d-flex align-items-center'}>{val.displayName}</Col>
                                    <Col sm={8}>
                                        <Row>
                                            <Col>
                                                <Button variant={'outline-primary'}
                                                        onClick={() => replyToUser(val.user, true)}>
                                                    <Icon.Check color={'green'} height={25}/>
                                                </Button>
                                            </Col>
                                            <Col>
                                                <Button variant={'outline-primary'}
                                                        onClick={() => replyToUser(val.user, true)}>
                                                    <Icon.X color={'red'} height={25}/>
                                                </Button>
                                            </Col>
                                        </Row>
                                    </Col>
                                </Row>
                            </ListGroup.Item>
                        )
                    )
                }
            </ListGroup>
        </Container>
    );
};

export default AcceptPendingMembers;