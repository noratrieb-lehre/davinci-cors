import React from 'react';
import 'bootstrap/dist/css/bootstrap.min.css';
import "react-datetime/css/react-datetime.css";
import Router from "./site/Router";
import {BrowserRouter} from "react-router-dom";


function App() {
    return (
        <BrowserRouter>
            <Router/>
        </BrowserRouter>
    )
}

export default App;
