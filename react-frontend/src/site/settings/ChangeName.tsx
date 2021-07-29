import React, {useContext} from 'react';
import {Button, Container, Form, FormControl, FormGroup, FormLabel} from "react-bootstrap";
import {useFormik} from "formik";
import * as Yup from 'yup'
import {UserServiceContext} from "../Router";
import {CurrentClass} from "../mainsite/currentClass/ClassView";

const validationScheme = Yup.object().shape({
    'displayName': Yup.string()
        .max(50, 'Der Benutzername kann nicht länger als 50 Zeichen sein')
        .required('Der Benutzername darf nicht leer sein')
})

const ChangeName = () => {
    const userService = useContext(UserServiceContext)
    const currentClass = useContext(CurrentClass);

    const handleSubmit = ({displayName}: { displayName: string }) => {
        userService.updateOwnDisplayName(currentClass!.id, displayName)
    }

    const formik = useFormik({
        initialValues: {
            displayName: ''
        },
        onSubmit: handleSubmit,
        validateOnChange: false,
        validateOnBlur: true,
        validationSchema: validationScheme
    })

    return (
        <Container>
            <Form onSubmit={(e) => {
                e.preventDefault();
                formik.handleSubmit(e);
            }}>
                <FormGroup>
                    <FormLabel>Neuer Benutzername</FormLabel>
                    <FormControl type={'text'} name={'displayName'} onChange={formik.handleChange}/>
                </FormGroup>
                <br/>
                <Button type={'submit'}>Namen ändern</Button>
            </Form>
        </Container>
    );
};

export default ChangeName;