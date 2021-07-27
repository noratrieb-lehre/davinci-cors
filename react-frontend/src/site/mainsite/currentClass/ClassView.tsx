import React, {useContext, useEffect, useState} from 'react';
import {Container, Tab, Tabs} from "react-bootstrap";
import {Redirect, Route, Switch, useHistory, useParams} from 'react-router-dom';
import ClassInfo from "./ClassInfo";
import Timetable from "./timetable/Timetable";
import Calendar from "./calendar/Calendar";
import {UserServiceContext} from "../../Router";
import Class from "../../../data/class/Class";
import WieLangeNoch from "./wielangenoch/WieLangeNoch";
import AdminPanel from "./adminPanel/AdminPanel";

const CurrentClass = React.createContext<Class | undefined>(undefined)

const ClassView = () => {
    const {id} = useParams<{ id: string }>();
    const [currentClass, setCurrentClass] = useState<Class>();
    const [selectedSite, setSelectedSite] = useState<string>('info');
    const [isAdmin, setIsAdmin] = useState(false);
    const history = useHistory();
    const userService = useContext(UserServiceContext);
    useEffect(() => {
        if (id) {
            userService.getClass(id).then(setCurrentClass);
        }
    }, [id, userService])

    useEffect(() => {
        if (currentClass){
            setIsAdmin(userService.isAdmin(currentClass))
        }
    }, [currentClass])

    const onTabSelect = (key: string | null) => {
        if (key) {
            setSelectedSite(key);
        }
    }
    useEffect(() => {
        if (id && currentClass) {
            history.push(`/class/${id}/${selectedSite}`);
        }
    }, [history, id, selectedSite, currentClass])

    return (
        <Container fluid>

            <CurrentClass.Provider value={currentClass}>
                {
                    currentClass && (
                        <>
                            <Tabs id={'classview-tab'} className={'mb-3'} activeKey={selectedSite}
                                  onSelect={onTabSelect} sm={8} transition={false}>
                                <Tab eventKey={'info'} title={'Info'}/>
                                <Tab eventKey={"timetable"} title={'Stundenplan'}/>
                                <Tab eventKey={'calendar'} title={'Kalender'}/>
                                <Tab title={'Wie Lange Noch'} eventKey={'wielangenoch'}/>
                                {
                                    isAdmin && <Tab title={'Admin'} eventKey={'admin'}/>
                                }
                            </Tabs>

                            <Switch>
                                <Route path={'/class/:id/info'} component={ClassInfo}/>
                                <Route path={'/class/:id/calendar'} component={Calendar}/>
                                <Route path={'/class/:id/timetable'} component={Timetable}/>
                                <Route path={'/class/:id/wielangenoch'} component={WieLangeNoch}/>
                                {
                                    isAdmin &&
                                    <Route path={'/class/:id/admin'} component={AdminPanel}/>
                                }
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