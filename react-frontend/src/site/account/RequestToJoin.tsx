import React from 'react';
import {useFormik} from "formik";

const RequestToJoin = () => {
    const formik = useFormik({
        initialValues: {
            'id': ''
        }
    })
    return (
        <div>

        </div>
    );
};

export default RequestToJoin;