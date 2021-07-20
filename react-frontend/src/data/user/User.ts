import Class from "../class/Class";

export default interface User {
    "id": string,
    "email": string,
    "description": string,
    "classes"?: Array<Class>
}

export type PostUser = User & { password: string }