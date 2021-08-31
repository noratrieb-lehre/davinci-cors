import React, {useContext} from 'react';
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
import {Moment} from "moment";
import 'moment/locale/de-ch'
import {useFormik} from "formik";
import {UserServiceContext} from "../../../Router";
import {CurrentClass} from "../ClassView";
import * as Yup from "yup";
import EventType from "../../../../data/event/EventType";

type submitValues = {
    eventName: string,
    startDate: number,
    endDate?: number,
    description: string,
    eventType: EventType,
    notification: number | null
}

const validationSchema = Yup.object().shape({
    eventName: Yup.string()
        .max(50, 'Der Name darf maximal 50 Zeichen lang sein')
        .required('Das Event muss einen Namen haben'),
    startDate: Yup.string()
        .required('Das Startdatum muss definiert sein'),
    endDate: Yup.number()
        .moreThan(Yup.ref('startDate'), 'Das Endatum muss nach dem Stardatum sein')
        .notRequired(),
    description: Yup.string()
        .notRequired(),
    eventType: Yup.string()
        .oneOf(['homework', 'holidays', 'exam', 'other'])
        .required('Der Typ muss ausgewählt sein'),
})

const NewEvent = () => {
    const userService = useContext(UserServiceContext);
    const currentClass = useContext(CurrentClass);
    const onSubmit = ({eventName, startDate, endDate, description, eventType, notification}: submitValues) => {
        userService.createEvent(currentClass!.id, {
            name: eventName,
            start: startDate * 1000,
            end: (endDate) ? endDate * 1000 : undefined,
            description: description,
            type: eventType,
            notification: notification ? notification * 1000 : null
        })
    }

    const formik = useFormik({
        initialValues: {
            eventName: '',
            startDate: 0,
            endDate: 0,
            description: '',
            eventType: 'other',
            notification: null
        },
        onSubmit: onSubmit,
        validateOnBlur: true,
        validateOnChange: false,
        validationSchema: validationSchema
    })

    return (
        <Container>
            <ModalTitle>Event erstellen</ModalTitle>
            <br/>
            <Form onSubmit={(e) => {
                e.preventDefault();
                formik.handleSubmit(e);
            }}>
                <Row>
                    <Col>
                        <FormGroup>
                            <FormLabel>Name des Events</FormLabel>
                            <FormControl type="text" name="eventName" onChange={formik.handleChange}/>
                        </FormGroup>
                        <Alert variant={'danger'} show={!!formik.errors.eventName}>{formik.errors.eventName}</Alert>
                    </Col>
                </Row>
                <Row>
                    <Col>
                        <FormGroup>
                            <FormLabel>Startdatum</FormLabel>
                            <Datetime dateFormat={'DD.MM.YYYY'} timeFormat={'HH:mm'} locale={'de-ch'}
                                      onChange={moment => {
                                          if (typeof moment !== 'string')
                                              formik.setFieldValue('startDate', (moment as Moment).unix())
                                      }}/>
                        </FormGroup>
                        <Alert variant={'danger'} show={!!formik.errors.startDate}>{formik.errors.startDate}</Alert>
                    </Col>
                    <Col>
                        <FormGroup>
                            <FormLabel>Enddatum</FormLabel>
                            <Datetime dateFormat={'DD.MM.YYYY'} timeFormat={'HH:mm'} locale={'de-ch'}
                                      onChange={moment => {
                                          if (typeof moment !== 'string')
                                              formik.setFieldValue('endDate', (moment as Moment).unix())
                                      }}/>
                        </FormGroup>
                        <Alert variant={'danger'} show={!!formik.errors.endDate}>{formik.errors.endDate}</Alert>
                    </Col>

                    <FormGroup>
                        <FormLabel>Beschreibung</FormLabel>
                        <FormControl as={'textarea'} name={'description'} rows={10}
                                     style={{resize: 'none', overflowY: 'auto'}} onChange={formik.handleChange}/>
                    </FormGroup>
                    <Alert variant={'danger'} show={!!formik.errors.description}>{formik.errors.description}</Alert>
                </Row>
                <Row>
                    <FormGroup>
                        <FormLabel>Benachrichtigung (optional)</FormLabel>
                        <Datetime dateFormat={'DD.MM.YYYY'} timeFormat={'HH:mm'} locale={'de-ch'}
                                  onChange={moment => {
                                      if (typeof moment !== 'string')
                                          formik.setFieldValue('notification', (moment as Moment).unix())
                                  }}/>
                    </FormGroup>
                </Row>
                <br/>
                <Row className={'text-center'}>
                    <Dropdown onSelect={(value) => formik.setFieldValue('eventType', value)}>
                        <Dropdown.Toggle>{getFormatted(formik.values.eventType)}</Dropdown.Toggle>
                        <Dropdown.Menu>
                            <Dropdown.Item eventKey={'homework'}>Hausaufgabe</Dropdown.Item>
                            <Dropdown.Item eventKey={'exam'}>Prüfung</Dropdown.Item>
                            <Dropdown.Item eventKey={'holidays'}>Ferien</Dropdown.Item>
                            <Dropdown.Item eventKey={'other'} default>Anderes</Dropdown.Item>
                        </Dropdown.Menu>
                    </Dropdown>
                </Row>
                <br/>
                <Row className={'text-center'}>
                    <Col>
                        <Button type={'submit'}>Speichern</Button>
                    </Col>
                </Row>
            </Form>

        </Container>
    );
};

const getFormatted = (type: string): string => types[type];

const types: { [type: string]: string } = {
    'homework': 'Hausaufgabe',
    'holidays': 'Ferien',
    'exam': 'Prüfung',
    'other': 'Anderes',
}

export default NewEvent;