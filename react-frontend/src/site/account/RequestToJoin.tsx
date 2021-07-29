import React, {useContext, useEffect, useState} from 'react';
import {Container, ModalBody, ModalTitle} from "react-bootstrap";
import {UserServiceContext} from "../Router";
import {useParams} from "react-router-dom";

const possibleMessage: { [key: string]: { title: string, body: string } } = {
    'success': {
        title: 'Deine Anfrage wurde verschickt',
        body: 'Jetzt musst du nur noch warten bis ein Admin deine Anfrage annimmt'
    },
    'not-authorized': {
        title: 'Es gab einen Fehler mit der Authentifizierung',
        body: 'Versuche mal dich wieder ab- und dann wieder an-zumelden'
    },
    'already-joined': {
        title: 'Deine Anfrage wurde bereits verschickt',
        body: 'Überprüfe mal, ob du schon in der Klasse bist. Wenn nicht, musst du auf die Besättigung eines Admins warten'
    },
    'other-error': {
        title: 'Es gab einen Fehler bei der Verarbeitung',
        body: 'Versuche es später wieder'
    },
    'class-not-found': {
        title: 'Die Klasse wurde nicht gefunden',
        body: 'Vergewissere dich, dass du den Link richtig kopiert hast'
    }
}

const RequestToJoin = () => {
    const {id} = useParams<{ id: string }>()
    const userService = useContext(UserServiceContext);
    const [message, setMessage] = useState({
        title: '',
        body: ''
    });

    const effect = () => {
        userService.requestToJoinClass(id).then(() => {
            setMessage(possibleMessage['success'])
        }).catch(err => {
            if (possibleMessage[err.message as string])
                setMessage(possibleMessage[err.message as string])
            else {
                switch (err.message) {
                    case 'token-expired':
                        userService.forceUpdate().then(() => effect())
                }
            }
        })
    }

    // eslint-disable-next-line
    useEffect(effect, [id])
    return (
        <Container>
            <ModalTitle>{message.title}</ModalTitle>
            <ModalBody>{message.body}</ModalBody>
        </Container>
    );
};

export default RequestToJoin;