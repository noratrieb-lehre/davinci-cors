import Event from "../data/event/Event";
import Axios from './AxiosInstance'

export default class EventRequest {
    private readonly axios: Axios;

    public constructor() {
        this.axios = Axios.getInstance();
    }

    public async createEvent(classID: string, event: Event): Promise<void> {
        await this.axios.axios.post(`/classes/${classID}/events`, {
            id: '',
            type: event.type,
            name: event.name,
            start: event.start,
            end: event.end,
            description: event.description
        })
    }
}