import React, {useContext} from 'react';
import {Button, Container, Form, FormControl, FormGroup, FormLabel, ModalBody, ModalTitle} from "react-bootstrap";
import {UserServiceContext} from "../Router";
import {useFormik} from "formik";
import * as Yup from "yup";

const validationScheme = Yup.object().shape({
    'snowflake': Yup.string()
        .max(20, 'Die ID kann nicht länger als 20 Zeichen sein')
        .required('Die ID darf nicht leer sein')
})

const LinkDiscord = () => {
    const userService = useContext(UserServiceContext);

    const handleSubmit = ({snowflake}: { snowflake: string }) => {
        userService.linkAccountToDiscord(snowflake);
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
        <Container>
            <ModalTitle>Discord Account mit CORS Account verbinden</ModalTitle>
            <ModalBody>Die Discord ID wird benötigt, um Benutzer zu authentifizeren, damit der Discord Bot weiss, wer welche Einstellungen vornehmen kann. Wenn du nicht weisst wie man
                eine Discord ID bekommt, kannst du <a href={'https://discordzoom.com/blog/discord-user-id/'}>diesem Link</a> folgen</ModalBody>
            <Form onSubmit={(e) => {
                e.preventDefault();
                formik.handleSubmit(e);
            }}>
                <FormGroup>
                    <FormLabel>Discord ID</FormLabel>
                    <FormControl type={'text'} name={'snowflake'} onChange={formik.handleChange}/>
                </FormGroup>
                <br/>
                <Button type={'submit'}>Discord verbinden</Button>
            </Form>
        </Container>
    );
};

export default LinkDiscord;