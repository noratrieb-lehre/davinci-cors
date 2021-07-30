import {AxiosResponse} from "axios";
import TimeTableDay from "../data/timetable/TimetableDay";
import TimeTable from "../data/timetable/TimeTable";
import Axios from './AxiosInstance';
import Lesson from "../data/timetable/Lesson";

export default class TimetableRequest {
    private readonly axios: Axios;

    public constructor() {
        this.axios = Axios.getInstance();
    }

    public async createTimetable(classID: string): Promise<void> {
        await this.axios.axios.post(`/classes/${classID}/timetable`).catch((err) => {
            throw new Error(err.response.data)
        })
    }

    public async updateTimetable(classId: string, timetableDay: TimeTableDay, day: number): Promise<void> {
        const timetable = await this.axios.axios.get<TimeTable>(`/classes/${classId}/timetable`).then(r => r.data).catch((err) => {
            throw new Error(err.response.data)
        });
        timetable[day] = timetableDay;
        await this.axios.axios.put(`/classes/${classId}/timetable`, timetable).catch((err) => {
            throw new Error(err.response.data)
        })
    }
    public async addLesson(classId: string, lesson: Lesson, day: number): Promise<void> {
        const timetable = await this.axios.axios.get<TimeTable>(`/classes/${classId}/timetable`).then(r => r.data).catch((err) => {
            throw new Error(err.response.data)
        });
        timetable[day].push(lesson);
        await this.axios.axios.put(`/classes/${classId}/timetable`, timetable).catch((err) => {
            throw new Error(err.response.data)
        })
    }

    public async getTimeTable(classId: string): Promise<AxiosResponse<TimeTable> | undefined> {
        return await this.axios.axios.get<TimeTable>(`/classes/${classId}/timetable`).catch(() => {
            console.log('Stundenplan nicht vorhanden')
            return undefined
        }).catch((err) => {
            throw new Error(err.response.data)
        });
    }

}