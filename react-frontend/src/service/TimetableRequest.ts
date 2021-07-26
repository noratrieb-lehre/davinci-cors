import {AxiosInstance, AxiosResponse} from "axios";
import TimeTableDay from "../data/timetable/TimetableDay";
import TimeTable from "../data/timetable/TimeTable";

export default class TimetableRequest {
    private axios: AxiosInstance;

    public constructor(axios: AxiosInstance) {
        this.axios = axios
    }


    public async createTimetable(classID: string): Promise<void> {
        await this.axios.post(`/classes/${classID}/timetable`)
    }

    public async updateTimetable(classId: string, timetableDay: TimeTableDay, day: number): Promise<void> {
        const timetable = await this.axios.get<TimeTable>(`/classes/${classId}/timetable`).then(r => r.data);
        timetable[day] = timetableDay;
        await this.axios.put(`/classes/${classId}/timetable`, timetable)
    }

    public async getTimeTable(classId: string): Promise<AxiosResponse<TimeTable | undefined>> {
        return await this.axios.get<TimeTable>(`/classes/${classId}/timetable`);
    }

}