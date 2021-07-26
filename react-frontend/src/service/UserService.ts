import axios, {AxiosInstance} from "axios";
import User, {PostUser} from "../data/user/User";
import Class from "../data/class/Class";
import Member from "../data/user/Member";
import MemberRole from "../data/user/MemberRole";
import Event from "../data/event/Event";
import TimeTable from "../data/timetable/TimeTable";
import TimeTableDay from "../data/timetable/TimetableDay";
import TimetableRequest from "./TimetableRequest";
import UserRequest from "./UserRequest";
import EventRequest from "./EventRequest";
import ClassRequest from "./ClassRequest";
import MemberRequest from "./MemberRequest";


export default class UserService {
    private axios: AxiosInstance;
    private onUserChangeHandler: Array<(user?: User) => void> = [];
    private readonly timetableRequest: TimetableRequest;
    private readonly userRequest: UserRequest;
    private readonly eventRequest: EventRequest;
    private readonly classRequest: ClassRequest;
    private readonly memberRequest: MemberRequest;
    private refreshToken?: string;
    private currentUserID?: string;

    public constructor() {
        this.axios = axios.create({
            baseURL: 'http://localhost:8080/api'
        });
        this.timetableRequest = new TimetableRequest(this.axios);
        this.userRequest = new UserRequest(this.axios);
        this.eventRequest = new EventRequest(this.axios);
        this.classRequest = new ClassRequest(this.axios);
        this.memberRequest = new MemberRequest(this.axios)
    }

    public async getCurrentUser(): Promise<User> {
        return await this.axios.get<User>('/users/me').then(r => r.data);
    }

    public async logout(): Promise<void> {
        this.triggerOnAuthStateChange(undefined);
    }

    public async login(email: string, password: string): Promise<void> {
        const loginResponse = await this.userRequest.login(email, password);
        this.currentUserID = loginResponse.data.userid;

        this.setToken(loginResponse.headers);
        this.updateToken(loginResponse.data.expires)
        this.triggerOnAuthStateChange(await this.getCurrentUser());
    }

    public async createAccount(user: PostUser): Promise<void> {
        const response = await this.userRequest.createAccount(user);
        this.setToken(response.headers)
        this.updateToken(response.data.expires);
        const newUser = await this.getCurrentUser();
        this.triggerOnAuthStateChange(newUser);
    }

    public isAdmin(currentClass: Class): boolean {
        const role = currentClass.members.filter(val => this.currentUserID === val.user)[0]?.role;
        return role === 'owner' || role === 'admin'
    }

    public async getClass(id: string): Promise<Class> {
        return await this.classRequest.getClass(id);
    }

    public getMember(memberList: Array<Member>, userID: string): Member | undefined {
        return memberList.filter(val => val.user === userID)[0];
    }

    public onUserChange(handler: (user?: User) => void) {
        this.onUserChangeHandler.push(handler)
    }

    public async getClasses(): Promise<Array<Class> | undefined> {
        return this.axios.get<User>('/users/me').then(r => r.data.classes);
    }

    public async getTimeTable(classId: string): Promise<TimeTable | undefined> {
        return (await this.timetableRequest.getTimeTable(classId))?.data;
    }

    public async createTimetable(classID: string): Promise<void> {
        return this.timetableRequest.createTimetable(classID);
    }

    public async updateTimetable(classId: string, timetableDay: TimeTableDay, day: number): Promise<void> {
        return this.timetableRequest.updateTimetable(classId, timetableDay, day);
    }

    public async getCalendar(classId: string): Promise<Array<Event>> {
        const response = await this.axios.get<Array<Event>>(`/classes/${classId}/events`);
        return response.data;
    }

    public async createEvent(classID: string, event: Event): Promise<void> {
        await this.eventRequest.createEvent(classID, event)
    }

    public async getPendingMembers(classId: string): Promise<Array<User>> {
        return await this.axios.get<Array<User>>(`/classes/${classId}/requests`).then(r => r.data);
    }


    public async replyToRequest(classID: string, userID: string, accept: boolean): Promise<void> {
        await this.memberRequest.replyToRequest(classID, userID, accept);
    }


    public getMemberRole(role: MemberRole): string {
        switch (role) {
            case "admin":
                return 'Administrator';
            case "member":
                return 'Mitglied';
            case "owner":
                return 'Eigentümer'
        }
    }

    private triggerOnAuthStateChange(user?: User) {
        this.onUserChangeHandler.forEach(handler => handler(user))
    }


    private updateToken(expireDate: number) {
        if (this.refreshToken) {
            const then = new Date(expireDate);
            setTimeout(async () => {
                const response = await this.axios.get('/token', {
                    baseURL: 'http://localhost:8080/api',
                    headers: {
                        'Authorization': this.refreshToken
                    }
                })
                this.setToken(response.headers)
                this.updateToken(response.data.expires);
            }, then.getTime() - Date.now())
        }
    }

    private setToken(header: any) {
        console.log(header)
        this.axios = axios.create({
            baseURL: 'http://localhost:8080/api',
            headers: {
                Authorization: (header['token'] || undefined!["token undefined"])
            }
        })
        this.refreshToken = (header['refresh-Token'] || undefined!["token undefined"]);
    }
}
