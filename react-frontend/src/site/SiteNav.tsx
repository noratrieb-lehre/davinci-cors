import React, {useContext, useState} from 'react';
import {Nav, Navbar} from "react-bootstrap";
import {NavLink} from "react-router-dom";
import {UserServerContext} from "./Router";
import UserService from "../service/UserService";

const SiteNav = () => {
    const [isLoggedIn, setIsLoggedIn] = useState(false);
    const userService = useContext<UserService>(UserServerContext);
    userService.onAuthStateChange((user) => setIsLoggedIn(!!user));

    return (
        <Navbar expand={'lg'}>
            <Navbar.Brand>Hugo</Navbar.Brand>
            <Navbar.Toggle aria-controls="basic-navbar-nav"/>
            <Navbar.Collapse id="basic-navbar-nav">
                <Nav className="mr-auto">
                    <NavLink to={'/'} exact className={'nav-link'}>Über</NavLink>
                    {
                        isLoggedIn ? (
                            <NavLink to={'/account'} className={'nav-link'}>Account</NavLink>

                        ) : (
                            <NavLink to={'/'} className={'nav-link'}>Login</NavLink>
                        )
                    }
                </Nav>
            </Navbar.Collapse>
        </Navbar>
    );
};

export default SiteNav;