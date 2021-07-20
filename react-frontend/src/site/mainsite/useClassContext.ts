import React, {useContext} from "react";
import {ClassContext} from "./MainSite";
import Class from "../../data/class/Class";

const useClassContext = (): [Class | undefined, React.Dispatch<Class | undefined>] => {
    const [state, setState] = useContext(ClassContext)!
    return [state, setState];
}

export default useClassContext