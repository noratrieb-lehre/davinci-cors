import {AxiosInstance} from "axios";
import Event from "../data/event/Event";

export default class EventRequest {
    private axios: AxiosInstance;

    public constructor(axios: AxiosInstance) {
        this.axios = axios
    }

    public async createEvent(classID: string, event: Event): Promise<void> {
        await this.axios.post(`/classes/${classID}/events`, {
            id: '',
            type: event.type,
            name: event.name,
            start: event.start,
            end: event.end,
            description: event.description
        })
    }
}