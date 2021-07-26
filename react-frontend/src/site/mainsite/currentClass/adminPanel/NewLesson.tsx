import React, {useContext} from 'react';
import {
    Button,
    Col,
    Container,
    Dropdown,
    Form,
    FormControl,
    FormGroup,
    FormLabel,
    ModalTitle,
    Row
} from "react-bootstrap";
import Datetime from 'react-datetime';
import {useFormik} from "formik";
import {UserServiceContext} from "../../../Router";

const NewLesson = () => {
    const userService = useContext(UserServiceContext);

    const onSubmit = ({
                          subject,
                          start,
                          end,
                          description,
                          day
                      }: { subject: string, start: string, end: string, description: string, day: number }) => {

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
        validationSchema: true
    })

    return (
        <Container>
            <ModalTitle>Neue Lektion hinzufügen</ModalTitle>
            <br/>
            <Form>
                <Row>
                    <Col>
                        <FormGroup>
                            <FormLabel>Name der Lektion</FormLabel>
                            <FormControl name="subject" type="text"/>
                        </FormGroup>
                    </Col>
                </Row>
                <Row>
                    <Col>
                        <FormGroup>
                            <FormLabel>Start der Lektion</FormLabel>
                            <Datetime dateFormat={false} timeFormat={true}/>
                        </FormGroup>
                    </Col>
                    <Col>
                        <FormGroup>
                            <FormLabel>Ende der Lektion</FormLabel>
                            <Datetime dateFormat={false} timeFormat={true}/>
                        </FormGroup>
                    </Col>
                </Row>
                <Row>
                    <Col>
                        <FormGroup>
                            <FormLabel>Beschreibung</FormLabel>
                            <FormControl as={'textarea'} name={'description'} rows={10}
                                         style={{resize: 'none', overflowY: 'auto'}}/>
                        </FormGroup>
                    </Col>
                </Row>
                <br/>
                <Row>
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
                    </Col>
                </Row>
                <br/>
                <Row className={'text-center'}>
                    <Col>
                        <Button>Neue Lektion</Button>
                    </Col>
                </Row>
            </Form>
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