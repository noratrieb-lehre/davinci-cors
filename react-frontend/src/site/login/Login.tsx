import React, {useContext, useState} from 'react';
import {
    Alert,
    Button,
    Container,
    Form,
    FormControl,
    FormGroup,
    FormLabel,
    ModalFooter,
    ModalTitle
} from "react-bootstrap";
import {useFormik} from "formik";
import * as Yup from 'yup'
import {Link, useHistory} from "react-router-dom";
import UserService from "../../service/UserService";
import {UserServiceContext} from "../Router";

const LoginSchema = Yup.object().shape({
    email: Yup.string()
        .max(1000, 'Die Beschreibung darf nicht länger als 1000 Zeichen sein')
        .email('Ungültige E-Mail Adresse')
        .required('E-Mail-Feld darf nicht leer sein.'),
    password: Yup.string()
        .max(50, 'Das Zeichen darf nicht länger als 50 Zeichen sein')
        .required('Passwort-Feld darf nicht leer sein')
})

const Login = () => {
    const history = useHistory();
    const userService = useContext<UserService>(UserServiceContext);
    const [error, setError] = useState<string>();
    const handleSumbit = ({email, password}: { email: string, password: string }) => {
        userService.login(email, password).then(() => history.push('/class')).catch(err => {
            if (err.message === 'invalid-email-password')
                setError('Email oder Passwort ungültig')
        });
    }
    const formik = useFormik({
        initialValues: {
            'email': '',
            'password': ''
        },
        onSubmit: handleSumbit,
        validationSchema: LoginSchema,
        validateOnBlur: true,
        validateOnChange: false
    })
    return (
        <Container>
            <ModalTitle>Log In</ModalTitle>
            <br/>
            <Form onSubmit={(e) => {
                e.preventDefault();
                formik.handleSubmit(e)
            }}>
                <FormGroup>
                    <FormLabel>E-Mail Adresse</FormLabel>
                    <FormControl type={'text'} name={'email'} onChange={formik.handleChange}
                                 value={formik.values.email} isInvalid={!!formik.errors.email || !!error}
                                 placeholder={'E-Mail Adresse'}/>
                    <Alert variant={'danger'}
                           show={!!formik.errors.email || !!error}>{formik.errors.email || error}</Alert>
                    <br/>
                </FormGroup>
                <FormGroup>
                    <FormLabel>Passwort</FormLabel>
                    <FormControl type={'password'} name={'password'} onChange={formik.handleChange}
                                 value={formik.values.password}
                                 isInvalid={!!formik.errors.password || !!error}
                                 placeholder={'Passwort'}
                    />
                    <Alert variant={'danger'}
                           show={!!formik.errors.password || !!error}>{formik.errors.password || error}</Alert>
                    <br/>
                </FormGroup>
                <br/>
                <Button type={'submit'}>Login</Button>
            </Form>
            <br/>
            <ModalFooter>Noch nicht registriert? <Link to={'/signup'}>Hier registrieren!</Link></ModalFooter>
        </Container>
    );
};

export default Login;