import React from 'react';
import ModalTitle from 'react-bootstrap/ModalTitle';
import Container from 'react-bootstrap/Container';
import ChangeClassName from "./classsettings/ChangeClassName";
import ChangeClassDescription from "./classsettings/ChangeClassDescription";
import CopyLink from "./classsettings/CopyLink";
import LinkWithDiscord from "./classsettings/LinkWithDiscord";
import DeleteClass from "./classsettings/deleteclass/DeleteClass";

const ChangeClassSettings = () => {
    return (
        <Container>
            <ModalTitle>Klasse bearbeiten</ModalTitle>
            <br/>
            <ChangeClassName/>
            <br/>
            <ChangeClassDescription/>
            <br/>
            <CopyLink/>
            <br/>
            <LinkWithDiscord/>
            <br/>
            <DeleteClass/>
        </Container>
    );
};

export default ChangeClassSettings;