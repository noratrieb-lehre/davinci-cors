import EventType from "./EventType";

export default interface Event {
    "type": EventType,
    "name": string,
    "start": number,
    "end"?: number | null,
    "description": string
}