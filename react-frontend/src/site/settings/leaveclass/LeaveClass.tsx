import React, {useContext, useEffect, useState} from 'react';
import {Button, Container} from "react-bootstrap";
import LeaveClassPopUp from "./LeaveClassPopUp";
import {CurrentClass} from "../../mainsite/currentClass/ClassView";
import {UserServiceContext} from "../../Router";

const LeaveClass = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);
    const [show, setShow] = useState(false);
    const [isOwner, setIsOwner] = useState(true);

    useEffect(() => {
        userService.getSelfInClass(currentClass!.id).then(val => setIsOwner(val.role === 'owner'))
        //eslint-disable-next-line
    }, [currentClass])

    useEffect(() => console.log(isOwner), [isOwner])

    return (
        <Container className={'text-center'}>
            {
                show && <LeaveClassPopUp onClose={() => setShow(false)}/>
            }
            <Button variant={'danger'} onClick={() => setShow(true)} disabled={isOwner}>Klasse verlassen</Button>
        </Container>
    );
};

export default LeaveClass;