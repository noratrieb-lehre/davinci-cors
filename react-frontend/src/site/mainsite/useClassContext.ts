import React, {useContext} from "react";
import {ClassContext} from "./MainSite";

const useClassContext = (): [string | undefined, React.Dispatch<string | undefined>] => {
    // @ts-ignore
    const [state, setState] = useContext(ClassContext)
    return [state, setState];
}

export default useClassContext