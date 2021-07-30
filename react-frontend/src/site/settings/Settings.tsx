import React from 'react';
import {Container} from "react-bootstrap";
import ChangeName from "./ChangeName";
import LeaveClass from "./leaveclass/LeaveClass";

const Settings = () => {
    return (
        <Container>
            <ChangeName/>
            <br/>
            <LeaveClass/>
        </Container>
    );
};

export default Settings;