import React, {useContext} from 'react';
import Table from 'react-bootstrap/Table';
import ModalBody from 'react-bootstrap/ModalBody';
import ModalTitle from 'react-bootstrap/ModalTitle';
import Container from 'react-bootstrap/Container';
import {UserServiceContext} from "../../Router";
import {CurrentClass} from "./ClassView";

const ClassInfo = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);
    return (
        <Container className={'text-center'}>
            <ModalTitle>{currentClass?.name || 'Aktuell keine Informationen'}</ModalTitle>
            <ModalBody>{currentClass?.description}</ModalBody>
            <ModalBody>Besitzer: {currentClass?.members.filter(val => val.role === 'owner')[0]?.displayName || ''}</ModalBody>
            <Table>
                <thead>
                <tr>
                    <th>Name</th>
                    <th>E-Mail</th>
                    <th>Rolle</th>
                </tr>
                </thead>
                <tbody>
                {
                    currentClass?.members.map(val => (
                        <tr key={val.user}>
                            <td>{val.displayName}</td>
                            <td>{val.email}</td>
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