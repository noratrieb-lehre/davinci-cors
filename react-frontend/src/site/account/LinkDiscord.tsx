import React, {useContext} from 'react';
import Button from 'react-bootstrap/Button';
import Container from 'react-bootstrap/Container';
import Form from 'react-bootstrap/Form';
import FormGroup from 'react-bootstrap/FormGroup';
import FormControl from "react-bootstrap/FormControl";
import FormLabel from 'react-bootstrap/FormLabel';
import ModalBody from 'react-bootstrap/ModalBody';
import ModalTitle from 'react-bootstrap/ModalTitle';
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
        userService.linkAccountToDiscord(snowflake).catch(err => {
            switch (err.message) {
                case 'token-expired':
                    userService.forceUpdate().then(() => handleSubmit({snowflake}))
            }
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
        <Container>
            <ModalTitle>Discord Account mit CORS Account verbinden</ModalTitle>
            <ModalBody>Die Discord ID wird benötigt, um Benutzer zu authentifizeren, damit der Discord Bot weiss, wer
                welche Einstellungen vornehmen kann. Wenn du nicht weisst wie man
                eine Discord ID bekommt, kannst du <a href={'https://discordzoom.com/blog/discord-user-id/'}>diesem
                    Link</a> folgen</ModalBody>
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