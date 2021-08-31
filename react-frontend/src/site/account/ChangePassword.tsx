import React, {useContext, useState} from 'react';
import Alert from 'react-bootstrap/Alert';
import Button from 'react-bootstrap/Button';
import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Form from 'react-bootstrap/Form';
import FormGroup from 'react-bootstrap/FormGroup';
import FormControl from "react-bootstrap/FormControl";
import FormLabel from 'react-bootstrap/FormLabel';
import Row from 'react-bootstrap/Row';
import * as Yup from 'yup';
import {useFormik} from "formik";
import {UserServiceContext} from "../Router";

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
    const userService = useContext(UserServiceContext);
    const [error, setError] = useState<string>();

    const onSubmit = ({oldPassword, password}: { oldPassword: string, password: string }) => {
        userService.changePassword(password, oldPassword).catch(err => {
            switch (err.message) {
                case 'token-expired':
                    userService.forceUpdate().then(() => onSubmit({oldPassword, password}))
                    break;
                case 'wrong-password':
                    setError('Das Passwort ist nicht korrekt');
                    break;
            }
        })
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
                                         onChange={formik.handleChange}
                                         isInvalid={!!formik.errors.oldPassword || !!error}/>
                            <Alert variant={'danger'}
                                   show={!!formik.errors.oldPassword || !!error}>{formik.errors.oldPassword || error}</Alert>
                        </FormGroup>
                    </Col>
                </Row>
                <br/>
                <Row>
                    <Col>
                        <FormGroup>
                            <FormLabel>Neues Passwort</FormLabel>
                            <FormControl name={'password'} placeholder={'Neues Passwort'}
                                         onChange={formik.handleChange} isInvalid={!!formik.errors.password}/>
                            <Alert variant={'danger'} show={!!formik.errors.password}>{formik.errors.password}</Alert>
                        </FormGroup>
                    </Col>
                    <Col>
                        <FormGroup>
                            <FormLabel>Neues Passwort bestätigen</FormLabel>
                            <FormControl name={'confirmPassword'} placeholder={'Passwort bestätigen'}
                                         onChange={formik.handleChange} isInvalid={!!formik.errors.confirmPassword}/>
                            <Alert variant={'danger'}
                                   show={!!formik.errors.confirmPassword}>{formik.errors.confirmPassword}</Alert>
                        </FormGroup>
                    </Col>
                </Row>
                <br/>
                <Row>
                    <Col><Button type={'submit'}>Passwort ändern</Button></Col>
                </Row>
            </Form>
        </Container>
    );
};

export default ChangePassword;