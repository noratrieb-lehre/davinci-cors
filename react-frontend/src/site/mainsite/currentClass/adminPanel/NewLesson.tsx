import React, {useContext, useEffect, useState} from 'react';
import Alert from 'react-bootstrap/Alert';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import FormGroup from 'react-bootstrap/FormGroup';
import FormControl from "react-bootstrap/FormControl";
import FormLabel from 'react-bootstrap/FormLabel';
import ModalTitle from 'react-bootstrap/ModalTitle';
import Dropdown from 'react-bootstrap/Dropdown';
import Container from 'react-bootstrap/Container';
import Col from 'react-bootstrap/Col';
import Row from 'react-bootstrap/Row';
import Datetime from 'react-datetime';
import {useFormik} from "formik";
import {UserServiceContext} from "../../../Router";
import TimeTable from "../../../../data/timetable/TimeTable";
import {CurrentClass} from "../ClassView";
import * as Yup from 'yup';
import {Moment} from "moment";

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

type SubmitValues = { subject: string, start: number, end: number, description: string, day: number }

const NewLesson = () => {
    const userService = useContext(UserServiceContext);
    const currentClass = useContext(CurrentClass);
    const [timetable, setTimetable] = useState<TimeTable | undefined>();
    const [success, setSuccess] = useState(false);

    useEffect(() => {
        userService.getTimeTable(currentClass!.id).then(setTimetable);
        // eslint-disable-next-line
    }, [currentClass])

    const onSubmit = ({subject, start, end, description, day}: SubmitValues) => {
        const todayZero = new Date();
        todayZero.setHours(0, 0, 0, 0);
        const startMs = (new Date(start).getTime() - todayZero.getTime());
        const endMs = (new Date(end).getTime() - todayZero.getTime());

        userService.addLesson(currentClass!.id, {
            subject,
            start: startMs,
            end: endMs,
            description
        }, day).then(() => {
            setSuccess(true);
            setTimeout(() => setSuccess(false), 1500)
        })
    }

    const formik = useFormik({
        initialValues: {
            subject: '',
            start: 0,
            end: 0,
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
                        <Form onSubmit={(e) => {
                            e.preventDefault();
                            formik.handleSubmit(e)
                        }}>
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
                                        <Datetime dateFormat={false} timeFormat={true} locale={'de-ch'}  onChange={(e) => {
                                            if(typeof e !== 'string') {
                                                formik.setFieldValue('start', (e as Moment).unix() * 1000)
                                            }
                                        }}/>
                                        <Alert variant={'danger'}
                                               show={!!formik.errors.start}>{formik.errors.start}</Alert>
                                    </FormGroup>
                                </Col>
                                <Col>
                                    <FormGroup>
                                        <FormLabel>Ende der Lektion</FormLabel>
                                        <Datetime dateFormat={false} timeFormat={true} locale={'de-ch'}
                                                  onChange={(e) => {
                                                      if(typeof e !== 'string') {
                                                          formik.setFieldValue('end', (e as Moment).unix() * 1000)
                                                      }
                                                  }}/>
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
                                    <Button type={'submit'}>Neue Lektion</Button>
                                </Col>
                            </Row>
                        </Form>
                        <Alert variant={'success'} show={success}>Lektion erfolgreich erstellt</Alert>
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