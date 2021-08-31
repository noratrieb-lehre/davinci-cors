import React, {useContext} from 'react';
import Button from 'react-bootstrap/Button';
import Col from 'react-bootstrap/Col';
import Form from 'react-bootstrap/Form';
import FormGroup from 'react-bootstrap/FormGroup';
import FormControl from "react-bootstrap/FormControl";
import FormLabel from 'react-bootstrap/FormLabel';
import Row from 'react-bootstrap/Row';
import {useFormik} from "formik";
import * as Yup from 'yup'
import {CurrentClass} from "../../ClassView";
import {UserServiceContext} from "../../../../Router";

const validationScheme = Yup.object().shape({
    'description': Yup.string()
        .max(1000, 'Die Beschreibung kann maximal 1000 Zeichen lang sein')
        .required('Das Passwort Feld muss ausgefüllt sein')
})

const ChangeClassDescription = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);

    const handleSubmit = ({description}: { description: string }) => {
        userService.changeClassDescription(currentClass!.id, description);
    }

    const formik = useFormik({
        initialValues: {
            'description': currentClass?.description || ''
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
                        <FormControl as={"textarea"} value={formik.values.description} name={'description'} onChange={formik.handleChange}/>
                    </FormGroup>
                </Col>
                <Col>
                    <Button type={'submit'}>Beschreibung ändern</Button>
                </Col>
            </Row>
        </Form>
    );
};

export default ChangeClassDescription;