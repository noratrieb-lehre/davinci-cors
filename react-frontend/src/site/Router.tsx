import React, {useState} from 'react';
import {Container} from 'react-bootstrap';
import SiteNav from "./SiteNav";
import UserService from "../service/UserService";
import Login from "./login/Login";
import {Route, Switch} from 'react-router-dom';
import Account from "./Account";
import MainSite from "./MainSite";
import SignUp from "./login/SignUp";

const userService = new UserService();
const UserServerContext = React.createContext<UserService>(userService);

const Router = () => {
    const [isLoggedIn, setIsLoggedIn] = useState(false);
    userService.onAuthStateChange((user) => setIsLoggedIn(!!user))
    return (
        <Container fluid>
            <UserServerContext.Provider value={userService}>
                <SiteNav/>
                {
                    isLoggedIn ? (
                        <Switch>
                            <Route path={'/account'} component={Account}/>
                            <Route path={'/classes'} component={MainSite}/>
                        </Switch>
                    ) : (
                        <Switch>
                            <Route exact path={'/'} component={Login}/>
                            <Route path={'/signup'} component={SignUp}/>
                        </Switch>
                    )
                }
            </UserServerContext.Provider>
        </Container>
    );
};

export default Router;
export {UserServerContext}