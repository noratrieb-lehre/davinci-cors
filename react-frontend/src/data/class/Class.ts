import Member from "../user/Member";

export default interface Class {
    "id": string,
    "members": Array<Member>,
    "name": string,
    "description": string
}