import React, {useContext, useState} from 'react';
import {Button, Container} from "react-bootstrap";
import CreatePopup from "./CreatePopup";
import {UserServiceContext} from "../Router";

const CreateClass = () => {
    const [showPopUp, setShowPopUp] = useState(false);
    const userService = useContext(UserServiceContext);

    const handleSubmit = ({name, description}: { name: string, description: string }) => {
        userService.createClass(name, description).then(() => setShowPopUp(false));
    }

    return (
        <Container>
            <CreatePopup show={showPopUp} onSubmit={handleSubmit}/>
            <Button onClick={() => {
                setShowPopUp(true)
            }}>Klasse erstellen</Button>
        </Container>
    );
};

export default CreateClass;