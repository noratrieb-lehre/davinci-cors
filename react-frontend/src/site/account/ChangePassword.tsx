import React from 'react';
import {Button, Col, Container, Form, FormControl, FormGroup, FormLabel, Row} from "react-bootstrap";
import * as Yup from 'yup';
import {useFormik} from "formik";

const validationScheme = Yup.object().shape({
    'password': Yup.string()
        .min(6, 'Das Passwort muss mindestens 6 Zeichen haben')
        .max(50, 'Das Passwort kann nicht länger als 50 Zeichen sein')
        .required('Das Passwort-Feld darf nicht leer sein'),
    'confirmPassword': Yup.string()
        .oneOf([Yup.ref('password'), null], 'Die Passwörter stimmem nicht überein'),
    'oldPassword': Yup.string()
        .required('Das alte Passwort muss ausgefüllt sein.')
})

const ChangePassword = () => {
    const onSubmit = ({oldPassword, password}: { oldPassword: string, password: string }) => {

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
            <Form onSubmit={(e) => {
                e.preventDefault();
                formik.handleSubmit(e);
            }}>
                <Row>
                    <Col>
                        <FormGroup>
                            <FormLabel>Altes Passwort</FormLabel>
                            <FormControl name={'oldPassword'} placeholder={'Altes Passwort'}
                                         onChange={formik.handleChange}/>
                        </FormGroup>
                    </Col>
                </Row>
                <br/>
                <Row>
                    <Col>
                        <FormGroup>
                            <FormLabel>Neues Passwort</FormLabel>
                            <FormControl name={'password'} placeholder={'Neues Passwort'}
                                         onChange={formik.handleChange}/>
                        </FormGroup>
                    </Col>
                    <Col>
                        <FormGroup>
                            <FormLabel>Neues Passwort bestätigen</FormLabel>
                            <FormControl name={'confirmPassword'} placeholder={'Passwort bestätigen'}
                                         onChange={formik.handleChange}/>
                        </FormGroup>
                    </Col>
                </Row>
                <br/>
                <Row>
                    <Col><Button>Passwort ändern</Button></Col>
                </Row>
            </Form>
        </Container>
    );
};

export default ChangePassword;