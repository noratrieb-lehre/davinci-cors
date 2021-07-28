import React, {useContext} from 'react';
import {Button, Form, FormControl, FormGroup, FormLabel, ModalBody, ModalTitle} from "react-bootstrap";
import {useFormik} from "formik";
import {CurrentClass} from "../../ClassView";
import {UserServiceContext} from "../../../../Router";
import * as Yup from 'yup';

const validationScheme = Yup.object().shape({
    'snowflake': Yup.string()
        .required('Die ID darf nicht leer sein')
})

const LinkWithDiscord = () => {
    const userService = useContext(UserServiceContext);
    const currentClass = useContext(CurrentClass);

    const handleSubmit = ({snowflake}: { snowflake: string }) => {
        userService.linkClassToGuild(currentClass!.id, snowflake);
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
                    <FormControl type={'text'} name={'snowflake'}/>
                </FormGroup>
                <br/>
                <Button type={'submit'}>Discord Server verbinden</Button>
            </Form>
        </>
    );
};

export default LinkWithDiscord;