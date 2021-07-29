import React, {useContext, useEffect, useState} from 'react';
import {
    Alert, Button, Col, Container,
    Dropdown, Form, FormControl, FormGroup, FormLabel, ModalTitle, Row
} from "react-bootstrap";
import Datetime from 'react-datetime';
import {useFormik} from "formik";
import {UserServiceContext} from "../../../Router";
import TimeTable from "../../../../data/timetable/TimeTable";
import {CurrentClass} from "../ClassView";
import * as Yup from 'yup';

const validationSchema = Yup.object().shape({
    'subject': Yup.string()
        .max(50, 'Der Name darf nicht länger als 50 Zeichen sein')
        .required('Der Name darf nicht leer sein'),
    'start': Yup.number()
        .required('Der Start darf nicht leer sein'),
    'end': Yup.number()
        .required('Das Ende darf nicht leer sein'),
    'description': Yup.string()
        .max(1000, 'Die Beschreibung darf nicht länger als 1000 Zeichen sein')
        .notRequired()

})

type SubmitValues = { subject: string, start: string, end: string, description: string, day: number }

const NewLesson = () => {
    const userService = useContext(UserServiceContext);
    const currentClass = useContext(CurrentClass);
    const [timetable, setTimetable] = useState<TimeTable | undefined>();

    useEffect(() => {
        userService.getTimeTable(currentClass!.id).then(setTimetable);
        // eslint-disable-next-line
    }, [currentClass])

    const onSubmit = ({subject, start, end, description, day}: SubmitValues) => {
    }

    const formik = useFormik({
        initialValues: {
            subject: '',
            start: '',
            end: '',
            description: '',
            day: 0
        },
        onSubmit: onSubmit,
        validateOnChange: false,
        validateOnBlur: true,
        validationSchema: validationSchema
    })

    return (
        <Container>
            {
                timetable && (
                    <>
                        <ModalTitle>Neue Lektion hinzufügen</ModalTitle>
                        <br/>
                        <Form>
                            <Row>
                                <Col>
                                    <FormGroup>
                                        <FormLabel>Name der Lektion</FormLabel>
                                        <FormControl name="subject" type="text" onChange={formik.handleChange}
                                                     isInvalid={!!formik.errors.subject}/>
                                        <Alert variant={'danger'}
                                               show={!!formik.errors.subject}>{formik.errors.subject}</Alert>
                                    </FormGroup>
                                </Col>
                            </Row>
                            <Row>
                                <Col>
                                    <FormGroup>
                                        <FormLabel>Start der Lektion</FormLabel>
                                        <Datetime dateFormat={false} timeFormat={true}/>
                                        <Alert variant={'danger'}
                                               show={!!formik.errors.start}>{formik.errors.start}</Alert>
                                    </FormGroup>
                                </Col>
                                <Col>
                                    <FormGroup>
                                        <FormLabel>Ende der Lektion</FormLabel>
                                        <Datetime dateFormat={false} timeFormat={true}
                                                  onChange={(e) => console.log(e)}/>
                                        <Alert variant={'danger'} show={!!formik.errors.end}>{formik.errors.end}</Alert>

                                    </FormGroup>
                                </Col>
                            </Row>
                            <Row>
                                <Col>
                                    <FormGroup>
                                        <FormLabel>Beschreibung</FormLabel>
                                        <FormControl as={'textarea'} name={'description'} rows={10}
                                                     style={{resize: 'none', overflowY: 'auto'}}
                                                     onChange={formik.handleChange}
                                                     isInvalid={!!formik.errors.description}/>
                                        <Alert variant={'danger'}
                                               show={!!formik.errors.description}>{formik.errors.description}</Alert>

                                    </FormGroup>
                                </Col>
                            </Row>
                            <br/>
                            <Row className={'text-center'}>
                                <Col>
                                    <Dropdown onSelect={(e) => formik.setFieldValue('day', Number(e))}>
                                        <Dropdown.Toggle>{getFormatted(formik.values.day)}</Dropdown.Toggle>
                                        <Dropdown.Menu>
                                            <Dropdown.Item eventKey={0}>Montag</Dropdown.Item>
                                            <Dropdown.Item eventKey={1}>Dienstag</Dropdown.Item>
                                            <Dropdown.Item eventKey={2}>Mittwoch</Dropdown.Item>
                                            <Dropdown.Item eventKey={3}>Donnerstag</Dropdown.Item>
                                            <Dropdown.Item eventKey={4}>Freitag</Dropdown.Item>
                                            <Dropdown.Item eventKey={5}>Samstag</Dropdown.Item>
                                            <Dropdown.Item eventKey={6}>Sonntag</Dropdown.Item>
                                        </Dropdown.Menu>
                                    </Dropdown>
                                    <Alert variant={'danger'} show={!!formik.errors.day}>{formik.errors.day}</Alert>
                                </Col>
                            </Row>
                            <br/>
                            <Row className={'text-center'}>
                                <Col>
                                    <Button>Neue Lektion</Button>
                                </Col>
                            </Row>
                        </Form>
                    </>
                )
            }
            {
                !timetable &&
                <Button onClick={() => userService.createTimetable(currentClass!.id)}>Stundenplan erstellen</Button>
            }
        </Container>
    );
};

const getFormatted = (type: number): string => types[type];

const types: { [type: number]: string } = {
    0: 'Montag',
    1: 'Dienstag',
    2: 'Mittwoch',
    3: 'Donnerstag',
    4: 'Freitag',
    5: 'Samstag',
    6: 'Sonntg',
}

export default NewLesson;