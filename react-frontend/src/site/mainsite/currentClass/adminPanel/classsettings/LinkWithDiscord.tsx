import React, {useContext, useState} from 'react';
import Alert from 'react-bootstrap/Alert';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import FormGroup from 'react-bootstrap/FormGroup';
import FormControl from "react-bootstrap/FormControl";
import FormLabel from 'react-bootstrap/FormLabel';
import ModalBody from 'react-bootstrap/ModalBody';
import ModalTitle from 'react-bootstrap/ModalTitle';
import {useFormik} from "formik";
import {CurrentClass} from "../../ClassView";
import {UserServiceContext} from "../../../../Router";
import * as Yup from 'yup';

const validationScheme = Yup.object().shape({
    'snowflake': Yup.string()
        .required('Die ID darf nicht leer sein')
})

const LinkWithDiscord = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);
    const [success, setSuccess] = useState(false);

    const handleSubmit = ({snowflake}: { snowflake: string }) => {
        userService.linkClassToGuild(currentClass!.id, snowflake).then(() => {
            setSuccess(true);
            new Promise((res) => setTimeout(res, 400)).then(() => setSuccess(false));
        });
    }

    const formik = useFormik({
        initialValues: {
            snowflake: ''
        },
        onSubmit: handleSubmit,
        validationSchema: validationScheme,
        validateOnChange: false,
        validateOnBlur: true
    })
    return (
        <>
            <ModalTitle>Klasse mit Discord-Server verbinden</ModalTitle>
            {
                currentClass?.discordId &&
                <b>Diese Klasse ist bereits mit einem Server verbunden. ({currentClass.discordId})</b>
            }
            <ModalBody>Damit die Mitglieder der Klasse Benachrichtigungen bekommen können, kann hier ein Discord-Server
                verbunden werden. Falls du nicht weisst, wie man eine Discord-Server ID bekommt, kannst
                du <a href={'https://support.discord.com/hc/de/articles/206346498-Wie-finde-ich-meine-Server-ID-'}>diesem
                    Link</a> folgen.</ModalBody>
            <Form onSubmit={(e) => {
                e.preventDefault();
                formik.handleSubmit(e);
            }}>
                <FormGroup>
                    <FormLabel>Discord-Server ID</FormLabel>
                    <FormControl type={'text'} name={'snowflake'} onChange={formik.handleChange}/>
                    <Alert show={!!formik.errors.snowflake} variant={'danger'}>{!!formik.errors.snowflake}</Alert>
                </FormGroup>
                <br/>
                <Button type={'submit'}>Discord Server verbinden</Button>
                <Alert show={success} variant={'success'}>Klasse erfolgreich mit Discord verbunden</Alert>
            </Form>
        </>
    );
};

export default LinkWithDiscord;