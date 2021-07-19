import React from 'react';
import 'bootstrap/dist/css/bootstrap.min.css';
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
