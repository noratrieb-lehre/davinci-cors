import React, {useContext} from 'react';
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
        .email('Ungültige E-Mail Adresse')
        .required('E-Mail-Feld darf nicht leer sein.'),
    password: Yup.string()
        .required('Passwort-Feld darf nicht leer sein')
})

const Login = () => {
    const history = useHistory()
    const userService = useContext<UserService>(UserServiceContext)
    const handleSumbit = ({email, password}: { email: string, password: string }) => {
        userService.login(email, password).then(() => history.push('/class'));

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
            <Form>
                <FormGroup>
                    <FormLabel>E-Mail Adresse</FormLabel>
                    <FormControl type={'text'} name={'email'} onChange={formik.handleChange}
                                 value={formik.values.email} isInvalid={!!formik.errors.email}
                                 placeholder={'E-Mail Adresse'}/>
                    <br/>
                    <Alert variant={'danger'} show={!!formik.errors.email}>{formik.errors.email}</Alert>
                </FormGroup>
                <FormGroup>
                    <FormLabel>Passwort</FormLabel>
                    <FormControl type={'password'} name={'password'} onChange={formik.handleChange}
                                 value={formik.values.password}
                                 isInvalid={!!formik.errors.password}
                                 placeholder={'Passwort'}
                    />
                    <br/>
                    <Alert variant={'danger'} show={!!formik.errors.password}>{formik.errors.password}</Alert>
                </FormGroup>
                <br/>
                <Button onClick={() => formik.submitForm()}>Login</Button>
            </Form>
            <br/>
            <ModalFooter>Noch nicht registriert? <Link to={'/signup'}>Hier registrieren!</Link></ModalFooter>
        </Container>
    );
};

export default Login;