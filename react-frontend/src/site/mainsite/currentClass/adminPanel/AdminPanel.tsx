import React from 'react';
import {Container} from "react-bootstrap";
import ChangeClassSettings from "./ChangeClassSettings";
import NewEvent from "./NewEvent";
import NewLesson from "./NewLesson";

const AdminPanel = () => {
    return (
        <Container>
            <ChangeClassSettings/>
            <hr/>
            <NewEvent/>
            <hr/>
            <NewLesson/>
        </Container>
    );
};

export default AdminPanel;