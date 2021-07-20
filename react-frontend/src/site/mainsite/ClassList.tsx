import React, {useContext, useEffect, useState} from 'react';
import {UserServiceContext} from "../Router";
import useClassContext from "./useClassContext";
import {ListGroup} from "react-bootstrap";

const ClassList = () => {
    const [allClasses, setAllClasses] = useState<Array<{name: string, id: string}>>([]);
    const [, setCurrentClass] = useClassContext();
    const userService = useContext(UserServiceContext);
    useEffect(() => {
        setAllClasses(userService.getClasses());
    }, [userService])

    const selectUserClass = (e: string | null) => {
        if(e) {
            setCurrentClass(userService.getClass(e));
        }
    }
    return (
        <ListGroup onSelect={selectUserClass}>
            {
                allClasses.map((val) => <ListGroup.Item eventKey={val.id} key={val.id}>{val.name}</ListGroup.Item>)
            }
        </ListGroup>
    );
}

export default ClassList;