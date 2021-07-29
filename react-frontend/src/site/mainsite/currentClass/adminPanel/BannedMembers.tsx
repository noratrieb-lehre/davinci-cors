import React, {useContext, useEffect, useState} from 'react';
import {Button, Col, Container, ListGroup, ModalTitle, Row} from "react-bootstrap";
import {CurrentClass} from "../ClassView";
import {UserServiceContext} from "../../../Router";
import * as Icon from 'react-bootstrap-icons';
import Member from "../../../../data/user/Member";

const BannedMembers = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);
    const [bannedMembers, setBannedMembers] = useState<Array<Member>>([]);

    const sendUnban = (member: Member) => {
        member.role = 'member';
        userService.updateClassMember(currentClass!.id, member).then(() =>
            setBannedMembers((arr) => arr.filter(val => val !== member))
        )
    }

    useEffect(() => {
        userService.getBannedMembers(currentClass!.id).then(setBannedMembers)
        // eslint-disable-next-line
    }, [])

    return (
        <Container>
            <ModalTitle>Gebannte Benutzer</ModalTitle>
            <br/>
            <ListGroup>
                {
                    bannedMembers?.map(val =>
                        <ListGroup.Item key={val.user}>
                            <Row>
                                <Col sm={5} className={'d-flex align-items-center'}>{val.displayName}</Col>
                                <Col sm={5} className={'d-flex align-items-center'}>{val.email}</Col>
                                <Col sm={1}>Entbannen</Col>
                                <Col sm={1}><Button variant={'outline-success'}
                                                    onClick={() => sendUnban(val)}><Icon.Check
                                    color={'green'} size={30}/></Button></Col>
                            </Row>
                        </ListGroup.Item>
                    )
                }
            </ListGroup>
            <br/>
        </Container>
    );
};

export default BannedMembers;