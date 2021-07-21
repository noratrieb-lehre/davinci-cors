import React, {useContext} from 'react';
import {Alert, Button, Col, Container, Form, FormControl, Row} from "react-bootstrap";
import {UserServiceContext} from "../Router";
import * as Yup from 'yup';
import {useFormik} from "formik";

const ChangeEmail = () => {
    const userService = useContext(UserServiceContext);
    const handleSubmit = ({email}: { email: string }) => {

    }

    const formik = useFormik({
        onSubmit: handleSubmit,
        validateOnChange: false,
        validateOnBlur: true,
        initialValues: {
            'email': userService.currentUser?.email || ''
        },
        validationSchema: Yup.object().shape({
            'email': Yup.string()
                .email('Bitte geben Sie eine valide E-Mail ein')
                .required('Das E-Mail Feld darf nicht leer sein')
        })
    })
    return (
        <Container>
            <br/>
            <h3 className={'modal-title'}>E-Mail ändern</h3>
            <br/>
            <Form onSubmit={e => {
                e.preventDefault();
                formik.handleSubmit(e);
            }} inline>
                <Row>
                    <Col>
                        <FormControl name={'email'} placeholder={'E-Mail Adresse ändern'}
                                     onChange={formik.handleChange} isInvalid={!!formik.errors.email}/>
                        <Alert variant={'danger'} show={!!formik.errors.email}>{formik.errors.email}</Alert>
                    </Col>
                    <Col>
                        <Button type={'submit'}>E-Mail ändern</Button>
                    </Col>
                </Row>
            </Form>
            <br/>
        </Container>
    );
};

export default ChangeEmail;