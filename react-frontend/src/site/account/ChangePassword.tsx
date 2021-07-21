import React from 'react';
import {Button, Col, Container, Form, FormControl, Row} from "react-bootstrap";
import * as Yup from 'yup';
import {useFormik} from "formik";

const validationScheme = Yup.object().shape({
    'password': Yup.string()
        .min(6, 'Das Passwort muss mindestens 6 Zeichen haben')
        .required('Das Passwort-Feld darf nicht leer sein'),
    'confirmPassword': Yup.string()
        .oneOf([Yup.ref('password'), null], 'Die Passwörter stimmem nicht überein'),
    'oldPassword': Yup.string()
        .required('Das alte Passwort muss ausgefüllt sein.')
})

const ChangePassword = () => {
    const onSubmit = ({oldPassword, password}: {oldPassword: string, password: string}) => {

    }
    const formik = useFormik({
        validationSchema: validationScheme,
        onSubmit: onSubmit,
        validateOnChange: false,
        validateOnBlur: true,
        initialValues: {
            'password': '',
            'confirmPassword': '',
            'oldPassword': ''
        }
    })
    return (
        <Container>
            <br/>
            <h3>Passwort ändern</h3>
            <br/>
            <Form>
                <Row>
                    <Col><FormControl name={'oldPassword'} placeholder={'Altes Passwort'}/></Col>
                </Row>
                <Row>
                    <Col><FormControl name={'password'} placeholder={'Neues Passwort'}/></Col>
                    <Col><FormControl name={'confirmPassword'} placeholder={'Passwort bestätigen'}/></Col>
                </Row>
                <Row>
                    <Col><Button>Passwort ändern</Button></Col>
                </Row>
            </Form>
        </Container>
    );
};

export default ChangePassword;