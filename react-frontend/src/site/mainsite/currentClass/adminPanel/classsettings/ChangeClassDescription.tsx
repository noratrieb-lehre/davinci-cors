import React from 'react';
import {Button, Col, Form, FormControl, FormGroup, FormLabel, Row} from "react-bootstrap";
import {useFormik} from "formik";
import * as Yup from 'yup'

const validationScheme = Yup.object().shape({
    'description': Yup.string()
        .max(1000, 'Die Beschreibung kann maximal 1000 Zeichen lang sein')
        .required('Das Passwort Feld muss ausgefüllt sein')
})

const ChangeClassDescription = () => {
    const handleSubmit = ({description}: { description: string }) => {

    }
    const formik = useFormik({
        initialValues: {
            'description': ''
        },
        onSubmit: handleSubmit,
        validateOnChange: false,
        validateOnBlur: true,
        validationSchema: validationScheme
    })
    return (
        <Form onSubmit={(e) => {
            e.preventDefault();
            formik.handleSubmit(e);
        }} inline>
            <Row>
                <FormLabel>Neue Klassenbeschreibung</FormLabel>
            </Row>
            <Row>
                <Col>
                    <FormGroup>
                        <FormControl as={"textarea"} name={'description'} onChange={formik.handleChange}/>
                    </FormGroup>
                </Col>
                <Col>
                    <Button type={'submit'}>Namen ändern</Button>
                </Col>
            </Row>
        </Form>
    );
};

export default ChangeClassDescription;