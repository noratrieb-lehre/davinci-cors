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
    const [self, setSelf] = useState<Member>();
    const [members, setMembers] = useState<Array<Member>>([]);

    useEffect(() => {
        userService.getMembers(currentClass!.id).then(arr => {
            const self = arr.filter(val => val.user === userService.currentUserID)[0]
            setSelf(self);
            setMembers(arr.filter(val => val.user !== self.user))
        })
        // eslint-disable-next-line
    }, [currentClass])
    return (
        <Container>
            {
                selectedMember && self && <EditUserPopUp member={selectedMember} selfRole={self.role}
                                                         onClose={() => setSelectedMember(undefined)}/>
            }
            <ModalTitle>Mitgliederverwaltung</ModalTitle>
            <br/>
            <ListGroup>
                {
                    members.map((member) => (
                        <ListGroup.Item>
                            <Row>
                                <Col sm={8} className={'d-flex align-items-center'}>
                                    <Col className={'d-flex align-items-center'} sm={2}>{member.displayName}</Col>
                                    <Col className={'d-flex align-items-center text-muted'} sm={6}>{member.email}</Col>
                                    <Col className={'d-flex align-items-center text-muted'}
                                         sm={4}>{userService.getMemberRole(member.role)}</Col>
                                </Col>
                                <Col sm={2}><Button variant={'outline-primary'}
                                                    onClick={() => setSelectedMember(member)}
                                                    disabled={userService.getRolesBelow(self!.role).indexOf(member.role) === -1}>Editieren</Button></Col>
                                <Col sm={2}>
                                    <Button variant={'outline-danger'}
                                            onClick={() => userService.deleteClassMember(currentClass!.id, member.user)}
                                            disabled={userService.getRolesBelow(self!.role).indexOf(member.role) === -1}>Kicken</Button>
                                </Col>
                            </Row>
                        </ListGroup.Item>
                    ))
                }
            </ListGroup>
        </Container>
    );
};

export default Members;