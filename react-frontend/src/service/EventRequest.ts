import Event from "../data/event/Event";
import Axios from './AxiosInstance'

export default class EventRequest {
    private readonly axios: Axios;

    public constructor() {
        this.axios = Axios.getInstance();
    }

    public async createEvent(classID: string, event: Event): Promise<void> {
        await this.axios.axios.post(`/classes/${classID}/events`, {
            type: event.type,
            name: event.name,
            start: event.start,
            end: event.end,
            description: event.description
        })
    }

    public async getCalendar(classId: string): Promise<Array<Event>> {
        const response = await this.axios.axios.get<Array<Event>>(`/classes/${classId}/events`);
        return response.data;
    }

    public async deleteEvent(classId: string, eventId: string) {
        await this.axios.axios.delete(`/classes/${classId}/events/${eventId}`)
    }
}