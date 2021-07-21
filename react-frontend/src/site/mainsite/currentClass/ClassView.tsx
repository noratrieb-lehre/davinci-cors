import React, {useContext, useEffect, useState} from 'react';
import {Container, Tab, Tabs} from "react-bootstrap";
import {Redirect, Route, Switch, useHistory, useParams} from 'react-router-dom';
import ClassInfo from "./ClassInfo";
import Timetable from "./Timetable";
import Calendar from "./calendar/Calendar";
import {UserServiceContext} from "../../Router";
import Class from "../../../data/class/Class";

const CurrentClass = React.createContext<Class | undefined>(undefined)

const ClassView = () => {
    const {id} = useParams<{ id: string }>();
    const [currentClass, setCurrentClass] = useState<Class>();
    const [selectedSite, setSelectedSite] = useState<string>('info');
    const history = useHistory();
    const userService = useContext(UserServiceContext);
    useEffect(() => {
        if (id) {
            const currentClass = userService.getClass(id)
            setCurrentClass(currentClass);
            console.log(currentClass)
        }
    }, [id, userService])

    const onTabSelect = (key: string | null) => {
        if (key) {
            setSelectedSite(key);
        }
    }
    useEffect(() => {
        if (id && currentClass) {
            history.push(`/class/${id}/${selectedSite}`);
        }
    }, [selectedSite, currentClass])

    return (
        <Container>


            <CurrentClass.Provider value={currentClass}>
                <Tabs id={'classview-tab'} className={'mb-3'} activeKey={selectedSite}
                      onSelect={onTabSelect}>
                    <Tab eventKey={'info'} title={'Info'}/>
                    <Tab eventKey={"timetable"} title={'Stundenplan'}/>
                    <Tab eventKey={'calendar'} title={'Kalender'}/>
                </Tabs>
                {
                    currentClass && (
                        <>
                            <Switch>
                                <Route path={'/class/:id/info'} component={ClassInfo}/>
                                <Route path={'/class/:id/calendar'} component={Calendar}/>
                                <Route path={'/class/:id/timetable'} component={Timetable}/>
                            </Switch>

                            <Redirect exact from={'/class/:id/'} to={`/class/${id}/info`}/>
                        </>
                    )
                }
            </CurrentClass.Provider>
        </Container>
    );
};


export default ClassView;
export {CurrentClass}