import React, {useContext} from 'react';
import {Button, Modal} from "react-bootstrap";
import {UserServiceContext} from "../../../../../Router";
import {CurrentClass} from "../../../ClassView";

const DeleteClassPopup = ({onClose}: {onClose: () => void}) => {
    const userService = useContext(UserServiceContext);
    const currentClass = useContext(CurrentClass);

    const handleDeleteClass = () => {
        userService.deleteClass(currentClass!.id)
    }

    return (
        <Modal show={true}>
            <Modal.Body>
                <Modal.Title>Klasse löschen</Modal.Title>
                <Modal.Body>Wenn du die Klasse löschst, kann diese Aktion nicht mehr rückgänig gemacht werden. Bist du dir sicher?</Modal.Body>
            </Modal.Body>
            <Modal.Footer>
                <Button variant={'secondary'} onClick={() => onClose()}>Abbrechen</Button>
                <Button variant={'danger'} onClick={handleDeleteClass}>Klasse löschen</Button>
            </Modal.Footer>
        </Modal>
    );
};

export default DeleteClassPopup;