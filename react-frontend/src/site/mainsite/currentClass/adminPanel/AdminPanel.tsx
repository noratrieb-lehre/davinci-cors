import React from 'react';
import {Container} from "react-bootstrap";
import ChangeClassSettings from "./ChangeClassSettings";
import NewEvent from "./NewEvent";
import NewLesson from "./NewLesson";
import PendingMembers from "./PendingMembers";
import Members from "./edituser/Members";

const AdminPanel = () => {
    return (
        <Container>
            <ChangeClassSettings/>
            <hr/>
            <NewEvent/>
            <hr/>
            <NewLesson/>
            <hr/>
            <Members/>
            <PendingMembers/>
        </Container>
    );
};

export default AdminPanel;