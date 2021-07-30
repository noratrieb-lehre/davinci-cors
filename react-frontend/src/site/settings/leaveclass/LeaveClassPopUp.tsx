import React, {useContext} from 'react';
import {Button, Modal} from "react-bootstrap";
import {UserServiceContext} from "../../Router";
import {CurrentClass} from "../../mainsite/currentClass/ClassView";

const LeaveClassPopUp = ({onClose}: {onClose: () => void}) => {
    const userService = useContext(UserServiceContext);
    const currentClass = useContext(CurrentClass);

    const handleLeaveClass = () => {
        userService.deleteSelfInClass(currentClass!.id);
    }

    return (
        <Modal show={true}>
            <Modal.Title>Klasse verlassen</Modal.Title>
            <Modal.Body>Bist du sicher das du die Klasse verlassen willst? Du kannst sie jederzeit wieder beitreten.</Modal.Body>
            <Modal.Footer>
                <Button variant={'secondary'} onClick={() => onClose()}>Abbrechen</Button>
                <Button variant={'danger'} onClick={handleLeaveClass}>Klasse verlassen</Button>
            </Modal.Footer>
        </Modal>
    );
};

export default LeaveClassPopUp;