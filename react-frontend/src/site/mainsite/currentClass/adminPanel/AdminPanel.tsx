import React from 'react';
import {Container} from "react-bootstrap";
import ChangeClassSettings from "./ChangeClassSettings";
import NewEvent from "./NewEvent";
import NewLesson from "./NewLesson";
import PendingMembers from "./PendingMembers";

const AdminPanel = () => {
    return (
        <Container>
            <ChangeClassSettings/>
            <hr/>
            <NewEvent/>
            <hr/>
            <NewLesson/>
            <PendingMembers/>
        </Container>
    );
};

export default AdminPanel;