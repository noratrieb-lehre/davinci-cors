import React, {useContext} from 'react';
import {FormControl, ModalBody, ModalTitle} from "react-bootstrap";
import {CurrentClass} from "../../ClassView";

const CopyLink = () => {
    const currentClass = useContext(CurrentClass);
    return (
        <>
            <ModalTitle>Link für die Klasse</ModalTitle>
            <ModalBody>Damit andere dieser Klasse beitreten können, müssen sie zuerst eine Anfrage an die Klasse machen.
                Das können Sie mit folgendem Link:</ModalBody>
            <FormControl type={'text'}
                         value={`${window.location.protocol}//${window.location.host}/join/${currentClass!.id}`}/>
        </>
    );
};

export default CopyLink;