import React, {useContext} from 'react';
import Alert from 'react-bootstrap/Alert';
import Button from 'react-bootstrap/Button';
import Col from 'react-bootstrap/Col';
import Form from 'react-bootstrap/Form';
import FormGroup from 'react-bootstrap/FormGroup';
import FormControl from "react-bootstrap/FormControl";
import FormLabel from 'react-bootstrap/FormLabel';
import Row from 'react-bootstrap/Row';
import {useFormik} from "formik";
import * as Yup from 'yup'
import {UserServiceContext} from "../../../../Router";
import {CurrentClass} from "../../ClassView";

const validationScheme = Yup.object().shape({
    'name': Yup.string()
        .max(50, 'Der Name darf maximal 50 Zeichen lang sein')
        .required('Der Name darf nicht leer sein')
})

const ChangeClassName = () => {
    const currentClass = useContext(CurrentClass);
    const userService = useContext(UserServiceContext);

    const handleSubmit = ({name}: { name: string }) => {
        userService.changeClassName(currentClass!.id, name);
    }

    const formik = useFormik({
        initialValues: {
            'name': currentClass?.name || ''
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
                <FormLabel>Neuer Klassename</FormLabel>
            </Row>
            <Row>
                <Col>
                    <FormGroup>
                        <FormControl type={'text'} value={formik.values.name} name={'name'}  onChange={formik.handleChange}/>
                        <Alert show={!!formik.errors.name} variant={'danger'}>{!!formik.errors.name}</Alert>
                    </FormGroup>
                </Col>
                <Col>
                    <Button type={'submit'}>Namen ändern</Button>
                </Col>
            </Row>
        </Form>
    );
};

export default ChangeClassName;