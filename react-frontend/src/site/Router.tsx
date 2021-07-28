import React, {useEffect, useState} from 'react';
import {Container} from 'react-bootstrap';
import SiteNav from "./SiteNav";
import UserService from "../service/UserService";
import Login from "./login/Login";
import {Route, Switch} from 'react-router-dom';
import Account from "./account/Account";
import MainSite from "./mainsite/MainSite";
import SignUp from "./login/SignUp";
import User from "../data/user/User";
import RequestToJoin from "./account/RequestToJoin";

const userService = new UserService();
const UserServiceContext = React.createContext<UserService>(userService);

const Router = () => {
    const [currentUser, setCurrentUser] = useState<User>();
    useEffect(() => {
        userService.onUserChange((user) => setCurrentUser(user))
    }, [])
    return (
        <Container fluid>
            <UserServiceContext.Provider value={userService}>
                <SiteNav/>
                {
                    !!currentUser ? (
                        <Switch>
                            <Route exact path={'/class'} component={MainSite}/>
                            <Route exact path={'/class/:id'} component={MainSite}/>
                            <Route path={'/class/:id/info'} component={MainSite}/>
                            <Route path={'/class/:id/calendar'} component={MainSite}/>
                            <Route path={'/class/:id/timetable'} component={MainSite}/>
                            <Route path={'/class/:id/wielangenoch'} component={MainSite}/>
                            <Route path={'/class/:id/admin'} component={MainSite}/>
                            <Route path={'/join/:id'} component={RequestToJoin}/>
                            <Route path={'/account'} component={Account}/>
                        </Switch>
                    ) : (
                        <Switch>
                            <Route exact path={'/'} component={Login}/>
                            <Route path={'/signup'} component={SignUp}/>
                        </Switch>
                    )
                }
            </UserServiceContext.Provider>
        </Container>
    );
};

export default Router;
export {UserServiceContext}