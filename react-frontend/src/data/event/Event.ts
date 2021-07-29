import EventType from "./EventType";

export default interface Event {
    "id"?: string,
    "type": EventType,
    "name": string,
    "start": number,
    "end"?: number | null,
    "description": string,
    "notification"?: number | null
}