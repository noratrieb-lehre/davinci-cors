import React, {useContext, useEffect, useState} from 'react';
import {UserServiceContext} from "../Router";
import useClassContext from "./useClassContext";
import {ListGroup} from "react-bootstrap";

const ClassList = () => {
    const [allClasses, setAllClasses] = useState<Array<{name: string, id: string}>>([]);
    const [currentClass, setCurrentClass] = useClassContext();
    const userService = useContext(UserServiceContext);
    useEffect(() => {
        setAllClasses(userService.getClasses());
    }, [])
    return (
        <ListGroup onSelect={(e) => setCurrentClass(e || '')}>
            {
                allClasses.map((val) => <ListGroup.Item eventKey={val.id} key={val.id}>{val.name}</ListGroup.Item>)
            }
        </ListGroup>
    );
}

export default ClassList;