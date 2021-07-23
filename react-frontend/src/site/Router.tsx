import React, {useState} from 'react';
import {Container} from 'react-bootstrap';
import SiteNav from "./SiteNav";
import UserService from "../service/UserService";
import Login from "./login/Login";
import {Redirect, Route, Switch} from 'react-router-dom';
import Account from "./account/Account";
import MainSite from "./mainsite/MainSite";
import SignUp from "./login/SignUp";

const userService = new UserService();
const UserServiceContext = React.createContext<UserService>(userService);

const Router = () => {
    const [isLoggedIn, setIsLoggedIn] = useState(!!userService.currentUser);
    userService.onUserChange((user) => setIsLoggedIn(!!user))
    return (
        <Container fluid>
            <UserServiceContext.Provider value={userService}>
                <SiteNav/>
                {
                    isLoggedIn ? (
                        <Switch>
                            <Route exact path={'/class'} component={MainSite}/>
                            <Route exact path={'/class/:id'} component={MainSite}/>
                            <Route path={'/class/:id/info'} component={MainSite}/>
                            <Route path={'/class/:id/calendar'} component={MainSite}/>
                            <Route path={'/class/:id/timetable'} component={MainSite}/>
                            <Route path={'/class/:id/wielangenoch'} component={MainSite}/>
                            <Route path={'/class/:id/admin'} component={MainSite}/>
                            <Route path={'/account'} component={Account}/>
                            <Route component={() => (<Redirect to={'/'}/>)}/>
                        </Switch>
                    ) : (
                        <Switch>
                            <Route exact path={'/'} component={Login}/>
                            <Route path={'/signup'} component={SignUp}/>
                            <Route component={() => (<Redirect to={'/'}/>)}/>
                        </Switch>
                    )
                }
            </UserServiceContext.Provider>
        </Container>
    );
};

export default Router;
export {UserServiceContext}