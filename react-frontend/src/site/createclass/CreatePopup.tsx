import React from 'react';
import Alert from 'react-bootstrap/Alert';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import FormGroup from 'react-bootstrap/FormGroup';
import FormControl from "react-bootstrap/FormControl";
import FormLabel from 'react-bootstrap/FormLabel';
import Modal from 'react-bootstrap/Modal'
import {useFormik} from "formik";
import * as Yup from 'yup';


const validationScheme = Yup.object().shape({
    'name': Yup.string()
        .max(50, 'Der Name darf nicht länger als 50 Zeichen sein')
        .required('Der Name darf nicht leer sein'),
    'description': Yup.string()
        .max(1000, 'Die Beschreibung darf nicht länger als 1000 Zeichen sein')
        .notRequired()
})

type Props = { show: boolean, onSubmit: ({name, description}: { name: string, description: string }) => void }

const CreatePopup = ({show, onSubmit}: Props) => {

    const formik = useFormik({
        initialValues: {
            'name': '',
            'description': ''
        },
        onSubmit: onSubmit,
        validationSchema: validationScheme,
        validateOnChange: false,
        validateOnBlur: true
    })

    return (
        <Modal show={show}>
            <Modal.Body>
                <Modal.Title>Klasse erstellen</Modal.Title>
                <Form onSubmit={(e) => {
                    e.preventDefault();
                    formik.handleSubmit(e);
                }}>
                    <FormGroup>
                        <FormLabel>Name der Klasse</FormLabel>
                        <FormControl type={'text'} placeholder={'Name der Klasse'} onChange={formik.handleChange}
                                     name={'name'} isInvalid={!!formik.errors.name}/>
                        <Alert show={!!formik.errors.name} variant={'danger'}>{formik.errors.name}</Alert>
                    </FormGroup>
                    <FormGroup>
                        <FormLabel>Beschreibung der Klasse</FormLabel>
                        <FormControl as={'textarea'} onChange={formik.handleChange}
                                     style={{resize: 'none', overflowY: 'auto'}} name={'description'}
                                     isInvalid={!!formik.errors.description}/>
                        <Alert show={!!formik.errors.description} variant={'danger'}>{formik.errors.description}</Alert>
                    </FormGroup>
                    <FormGroup className={'text-center'}>
                        <Button type={'submit'}>Klasse erstellen</Button>
                    </FormGroup>
                </Form>
            </Modal.Body>
        </Modal>
    );
};

export default CreatePopup;