import React, {useContext, useEffect, useState} from 'react';
import Button from 'react-bootstrap/Button';
import ModalTitle from 'react-bootstrap/ModalTitle';
import Container from 'react-bootstrap/Container';
import Col from 'react-bootstrap/Col';
import Row from 'react-bootstrap/Row';
import ListGroup from 'react-bootstrap/ListGroup';
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
                                <Col className={'d-flex align-items-center'}>{val.displayName}</Col>
                                <Col className={'d-flex align-items-center'}>{val.email}</Col>
                                <Col><Button variant={'outline-success'}
                                                    onClick={() => sendUnban(val)}><Icon.X
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