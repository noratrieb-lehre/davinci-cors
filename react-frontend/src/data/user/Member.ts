import UserRole from "./UserRole";

export default interface Member {
    "user": string,
    "class": string,
    "displayName": string
    "role": UserRole
}