import React, {useContext} from 'react';
import {Container, ModalBody, ModalTitle, Table} from "react-bootstrap";
import {UserServiceContext} from "../../Router";
import {CurrentClass} from "./ClassView";

const ClassInfo = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);
    return (
        <Container className={'text-center'}>
            <ModalTitle>{currentClass!.name}</ModalTitle>
            <ModalBody>{currentClass?.description}</ModalBody>
            <ModalBody>Besitzer: {userService.getMember(currentClass!.members, currentClass!.owner)?.displayName || 'Aktuell keine Information'}</ModalBody>
            <Table>
                <thead>
                <tr>
                    <th>Name</th>
                    <th>Rolle</th>
                </tr>
                </thead>
                <tbody>
                {
                    currentClass!.members.map(val => (
                        <tr>
                            <td>{val.displayName}</td>
                            <td>{userService.getMemberRole(val.role)}</td>
                        </tr>
                    ))
                }
                </tbody>
            </Table>
        </Container>
    );
};

export default ClassInfo;