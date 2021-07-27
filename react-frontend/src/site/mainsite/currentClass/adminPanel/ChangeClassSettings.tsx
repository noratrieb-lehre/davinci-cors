import React from 'react';
import {Container, ModalTitle} from "react-bootstrap";
import ChangeClassName from "./classsettings/ChangeClassName";
import ChangeClassDescription from "./classsettings/ChangeClassDescription";

const ChangeClassSettings = () => {
    return (
        <Container>
            <ModalTitle>Klasse bearbeiten</ModalTitle>
            <br/>
            <ChangeClassName/>
            <br/>
            <ChangeClassDescription/>
        </Container>
    );
};

export default ChangeClassSettings;