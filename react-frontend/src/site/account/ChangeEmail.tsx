import React, {useContext, useEffect, useState} from 'react';
import {Alert, Button, Col, Container, Form, FormControl, FormGroup, FormLabel, Row} from "react-bootstrap";
import {UserServiceContext} from "../Router";
import * as Yup from 'yup';
import {useFormik} from "formik";
import User from "../../data/user/User";

const ChangeEmail = () => {
    const userService = useContext(UserServiceContext);
    const [currentUser, setCurrentUser] = useState<User>();

    useEffect(() => {
        userService.getCurrentUser().then(setCurrentUser);
    }, [])

    const handleSubmit = ({email}: { email: string }) => {

    }

    const formik = useFormik({
        onSubmit: handleSubmit,
        validateOnChange: false,
        validateOnBlur: true,
        initialValues: {
            'email': currentUser?.email || ''
        },
        validationSchema: Yup.object().shape({
            'email': Yup.string()
                .email('Bitte geben Sie eine valide E-Mail ein')
                .max(50, 'Die E-Mail darf maximal 50 Zeichen lang sein')
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
                        <FormGroup>
                            <FormLabel>E-Mail Adresse</FormLabel>
                            <FormControl name={'email'} placeholder={'E-Mail Adresse ändern'}
                                         onChange={formik.handleChange} isInvalid={!!formik.errors.email}/>
                        </FormGroup>
                        <Alert variant={'danger'} show={!!formik.errors.email}>{formik.errors.email}</Alert>
                    </Col>
                    <Col>
                        <br/>
                        <Button type={'submit'}>E-Mail ändern</Button>
                    </Col>
                </Row>
            </Form>
            <br/>
        </Container>
    );
};

export default ChangeEmail;