import React, {useContext, useEffect, useState} from 'react';
import {UserServiceContext} from "../../Router";
import ListGroup from 'react-bootstrap/ListGroup';
import {useHistory, useParams} from "react-router-dom";

const ClassList = () => {
    const {id} = useParams<{ id: string }>();
    const [allClasses, setAllClasses] = useState<Array<{ name: string, id: string }>>([]);
    const userService = useContext(UserServiceContext);
    const history = useHistory();

    useEffect(() => {
        userService.getClasses().then(val => val?.map(c => ({name: c.name, id: c.id})))
            .then(((val) => setAllClasses(val || [])));
    }, [userService])

    const selectUserClass = (e: string | null) => {
        if (e) {
            history.push(`/class/${e}/info`)
        }
    }

    useEffect(() => {
        if (!id && allClasses[0])
            history.push(`/class/${allClasses[0].id}/info`)
    }, [history, id, allClasses])

    return (
        <>
            <ListGroup onSelect={selectUserClass}>
                {
                    allClasses.map((val) => <ListGroup.Item eventKey={val.id} active={val.id === id} key={val.id}>{val.name}</ListGroup.Item>)
                }
            </ListGroup>
        </>
    );
}

export default ClassList;