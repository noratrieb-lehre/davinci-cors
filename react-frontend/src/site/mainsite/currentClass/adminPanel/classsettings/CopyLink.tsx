import React, {useContext, useRef, useState} from 'react';
import {FormControl, ModalBody, ModalTitle, Overlay, Tooltip} from "react-bootstrap";
import {CurrentClass} from "../../ClassView";

const CopyLink = () => {
    const pointer = useRef<HTMLInputElement>(null);
    const [show, setShow] = useState(false);
    const currentClass = useContext(CurrentClass);
    return (
        <>
            <ModalTitle>Link für die Klasse</ModalTitle>
            <ModalBody>Damit andere dieser Klasse beitreten können, müssen sie zuerst eine Anfrage an die Klasse machen.
                Das können Sie mit folgendem Link:</ModalBody>
            <FormControl type={'text'}
                         value={`${window.location.protocol}//${window.location.host}/join/${currentClass!.id}`}
                         readOnly={false} ref={pointer} onClick={() => {
                             if(navigator.clipboard) {
                                 navigator.clipboard.writeText(`${window.location.protocol}//${window.location.host}/join/${currentClass!.id}`).then(() => {
                                     setShow(true)
                                     setTimeout(() => setShow(false), 750);
                                 })
                             }
            }} isValid={show}/>
            <Overlay target={pointer.current} show={show} placement={'top'}>
                <Tooltip id={"copy-message"}>Link wurde kopiert!</Tooltip>
            </Overlay>
        </>
    );
};

export default CopyLink;