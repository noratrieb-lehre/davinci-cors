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
                        <ListGroup.Item key={member.user}>
                            <Row>
                                <Col sm={9} className={'d-flex align-items-center'}>
                                    <Col className={'d-flex align-items-center'}>{member.displayName}</Col>
                                    <Col className={'d-flex align-items-center text-muted'}>{member.email}</Col>
                                    <Col className={'d-flex align-items-center text-muted'}>{userService.getMemberRole(member.role)}</Col>
                                </Col>
                                <Col sm={3}>
                                    <Row>
                                        <Col><Button variant={'outline-primary'}
                                                     onClick={() => setSelectedMember(member)}
                                                     disabled={!userService.getRolesBelow(self!.role).includes(member.role)}>Editieren</Button></Col>
                                        <Col>
                                            <Button variant={'outline-danger'}
                                                    onClick={() => userService.deleteClassMember(currentClass!.id, member.user)}
                                                    disabled={!userService.getRolesBelow(self!.role).includes(member.role)}>Kicken</Button>
                                        </Col>
                                        <Col>
                                            <Button variant={'outline-danger'}
                                                    onClick={() => {
                                                        member.role = "banned";
                                                        userService.updateClassMember(currentClass!.id, member)
                                                    }}
                                                    disabled={!userService.getRolesBelow(self!.role).includes(member.role)}>Bannen</Button>
                                        </Col>

                                    </Row>

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