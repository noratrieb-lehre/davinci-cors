import React, {useContext, useEffect, useState} from 'react';
import {Button, Col, Container, ListGroup, ModalTitle, Row} from "react-bootstrap";
import Member from "../../../../../data/user/Member";
import {CurrentClass} from "../../ClassView";
import {UserServiceContext} from "../../../../Router";
import EditUserPopUp from "./EditUserPopUp";

const Members = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);
    const [selectedMember, setSelectedMember] = useState<Member>();
    const [members, setMembers] = useState<Array<Member>>([]);

    useEffect(() => {
        userService.getMembers(currentClass!.id).then(arr => {
            const self = (arr.filter(val => val.user === userService.currentUserID)[0]);
            const roles = userService.getRolesBelow(self.role);
            setMembers(arr.filter(val => roles.indexOf(val.role) !== -1))
        })
        // eslint-disable-next-line
    }, [currentClass])
    return (
        <Container>
            {
                selectedMember && <EditUserPopUp member={selectedMember} onClose={() => setSelectedMember(undefined)}/>
            }
            <ModalTitle>Mitgliederverwaltung</ModalTitle>
            <br/>
            <ListGroup>
                {
                    members.map((member) => (
                        <ListGroup.Item>
                            <Row>
                                <Col sm={8} className={'d-flex align-items-center'}>{member.displayName}</Col>
                                <Col sm={2}><Button variant={'outline-primary'}
                                                    onClick={() => setSelectedMember(member)}>Editieren</Button></Col>
                                <Col sm={2}><Button variant={'outline-danger'}>Kicken</Button></Col>
                            </Row>
                        </ListGroup.Item>
                    ))
                }
            </ListGroup>
        </Container>
    );
};

export default Members;