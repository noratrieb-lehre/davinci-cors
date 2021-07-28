import React, {useContext, useEffect, useState} from 'react';
import {Button, Nav, Navbar} from "react-bootstrap";
import {Link, NavLink} from "react-router-dom";
import {UserServiceContext} from "./Router";
import UserService from "../service/UserService";
import CORS from '../img/mainlogo.svg';


const SiteNav = () => {
    const userService = useContext<UserService>(UserServiceContext);
    const [isLoggedIn, setIsLoggedIn] = useState(false);
    useEffect(() => {
        userService.onUserChange((user) => {
            setIsLoggedIn(!!user)
        });
        // eslint-disable-next-line
    }, [])

    return (
        <Navbar expand={'lg'}>
            <Navbar.Brand><Link to={'/'}><img src={CORS} height={75} alt={'Logo von CORS'}/></Link></Navbar.Brand>
            <Navbar.Toggle aria-controls="basic-navbar-nav"/>
            <Navbar.Collapse id="basic-navbar-nav">
                <Nav className="me-auto">
                    {
                        isLoggedIn ? (
                            <>
                                <NavLink to={'/class'} className={'nav-link'}>Klassenansicht</NavLink>
                                <NavLink to={'/account'} className={'nav-link'}>Account</NavLink>
                            </>

                        ) : (
                            <>
                                <NavLink to={'/'} exact className={'nav-link'}>Login</NavLink>
                                <NavLink to={'/signup'} exact className={'nav-link'}>Registrieren</NavLink>
                            </>
                        )
                    }
                </Nav>
                {
                    isLoggedIn && <Nav>
                        <Nav.Item className={'navbar-right'}>
                            <Button variant={'outline-primary'} onClick={() => userService.logout()}>Ausloggen</Button>
                        </Nav.Item>
                    </Nav>
                }
            </Navbar.Collapse>
        </Navbar>
    );
};

export default SiteNav;