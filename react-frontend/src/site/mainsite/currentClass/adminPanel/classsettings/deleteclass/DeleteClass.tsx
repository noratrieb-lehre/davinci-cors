import React, {useContext, useEffect, useState} from 'react';
import Button from 'react-bootstrap/Button';
import Container from 'react-bootstrap/Container';
import DeleteClassPopup from "./DeleteClassPopup";
import {UserServiceContext} from "../../../../../Router";
import {CurrentClass} from "../../../ClassView";

const DeleteClass = () => {
    const userService = useContext(UserServiceContext);
    const currentClass = useContext(CurrentClass);
    const [show, setShow] = useState(false);
    const [isOwner, setIsOwner] = useState(false);

    useEffect(() => {
        userService.getSelfInClass(currentClass!.id).then(val => setIsOwner(val.role === 'owner'))
        //eslint-disable-next-line
    }, [currentClass])

    return (
        <Container className={'text-center'}>
            {
                show && <DeleteClassPopup onClose={() => setShow(false)}/>
            }
            <Button variant={'danger'} onClick={() => setShow(true)} disabled={!isOwner}>Klasse löschen</Button>
        </Container>
    );
};

export default DeleteClass;