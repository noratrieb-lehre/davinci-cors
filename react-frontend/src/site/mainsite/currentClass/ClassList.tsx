import React, {useContext, useEffect, useState} from 'react';
import {UserServiceContext} from "../../Router";
import {ListGroup} from "react-bootstrap";
import {useHistory, useParams} from "react-router-dom";

const ClassList = () => {
    const {id} = useParams<{id: string}>();
    const [allClasses, setAllClasses] = useState<Array<{ name: string, id: string }>>([]);
    const userService = useContext(UserServiceContext);
    const history = useHistory();
    useEffect(() => {
        userService.getClasses().then(val => val?.map(c => ({name: c.name, id: c.id})))
            .then(((val) => setAllClasses(val || [])));
    }, [userService])

    const selectUserClass = (e: string | null) => {
        if (e) {
            history.push(`/class/${e}/`)
        }
    }
    return (
        <ListGroup onSelect={selectUserClass} defaultActiveKey={id || ''}>
            {
                allClasses.map((val) => <ListGroup.Item eventKey={val.id} key={val.id}>{val.name}</ListGroup.Item>)
            }
        </ListGroup>
    );
}

export default ClassList;