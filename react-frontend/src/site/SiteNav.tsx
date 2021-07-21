import React, {useContext, useState} from 'react';
import {Nav, Navbar} from "react-bootstrap";
import {Link, NavLink} from "react-router-dom";
import {UserServiceContext} from "./Router";
import UserService from "../service/UserService";
import CORS from '../img/mainlogo.svg';


const SiteNav = () => {
    const userService = useContext<UserService>(UserServiceContext);
    const [isLoggedIn, setIsLoggedIn] = useState(!!userService.currentUser);
    userService.onUserChange((user) => setIsLoggedIn(!!user));

    return (
        <Navbar expand={'lg'}>
            <Navbar.Brand><Link to={'/'}><img src={CORS} height={75} alt={'Logo von CORS'}/></Link></Navbar.Brand>
            <Navbar.Toggle aria-controls="basic-navbar-nav"/>
            <Navbar.Collapse id="basic-navbar-nav">
                <Nav className="mr-auto">
                    {
                        isLoggedIn ? (
                            <>
                                <NavLink to={'/'} exact className={'nav-link'}>Über</NavLink>
                                <NavLink to={'/account'} className={'nav-link'}>Account</NavLink>
                                <NavLink to={'/class'} className={'nav-link'}>Klassenansicht</NavLink>
                            </>

                        ) : (
                            <>
                                <NavLink to={'/'} exact className={'nav-link'}>Login</NavLink>
                                <NavLink to={'/signup'} exact className={'nav-link'}>Registrieren</NavLink>

                            </>
                        )
                    }
                </Nav>
            </Navbar.Collapse>
        </Navbar>
    );
};

export default SiteNav;