import React, {useContext, useState} from 'react';
import {Container, ModalTitle} from "react-bootstrap";
import {UserServiceContext} from "../Router";

const Account = () => {
    const userService = useContext(UserServiceContext);
    const [currentUser, setCurrentUser] = useState(userService.currentUser)!;
    userService.onAuthStateChange(user => setCurrentUser(user))
    return (
        <Container>
            <ModalTitle>Account von {currentUser!.email}</ModalTitle>
        </Container>
    );
};

export default Account;