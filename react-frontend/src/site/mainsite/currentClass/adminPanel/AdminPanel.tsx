import React from 'react';
import {Container} from "react-bootstrap";
import ChangeClassName from "./ChangeClassName";
import NewEvent from "./NewEvent";
import NewLesson from "./NewLesson";

const AdminPanel = () => {
    return (
        <Container>
            <ChangeClassName/>
            <hr/>
            <NewEvent/>
            <hr/>
            <NewLesson/>
        </Container>
    );
};

export default AdminPanel;