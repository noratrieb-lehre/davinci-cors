import React, {useContext, useState} from 'react';
import {Container, ModalTitle} from "react-bootstrap";
import {UserServiceContext} from "../Router";
import ChangeEmail from "./ChangeEmail";
import ChangePassword from "./ChangePassword";

const Account = () => {
    const userService = useContext(UserServiceContext);
    const [currentUser, setCurrentUser] = useState(userService.currentUser)!;
    userService.onUserChange(user => setCurrentUser(user))
    return (
        <Container className={'text-center'}>
            <ModalTitle>Account von {currentUser!.email}</ModalTitle>
            <br/>
            <hr/>
            <ChangeEmail/>
            <hr/>
            <ChangePassword/>
        </Container>
    );
};

export default Account;