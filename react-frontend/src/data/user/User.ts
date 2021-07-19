import Class from "../class/Class";

export default interface User {
    "id": string,
    "name": string,
    "description": string,
    "classes"?: Array<Class>
}