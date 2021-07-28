import React, {useContext, useEffect, useState} from 'react';
import {Container, ModalTitle} from "react-bootstrap";
import {UserServiceContext} from "../Router";
import ChangeEmail from "./ChangeEmail";
import ChangePassword from "./ChangePassword";
import User from "../../data/user/User";

const Account = () => {
    const userService = useContext(UserServiceContext);
    const [currentUser, setCurrentUser] = useState<User | undefined>();
    useEffect(() => {
        userService.getCurrentUser().then(setCurrentUser)
        userService.onUserChange(user => setCurrentUser(user))
        // eslint-disable-next-line
    }, [])
    return (
        <Container className={'text-center'}>
            <ModalTitle>Account von {currentUser?.email}</ModalTitle>
            <br/>
            <hr/>
            <ChangeEmail/>
            <hr/>
            <ChangePassword/>
        </Container>
    );
};

export default Account;