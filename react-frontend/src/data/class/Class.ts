import User from "../user/User";

export default interface Class {
    "id": string,
    "owner": string,
    "members": Array<User>,
    "name": string,
    "description": string
}