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
import Axios from "./AxiosInstance";
import axios from "axios";
import DiscordRequest from "./DiscordRequest";


export default class UserService {
    private onUserChangeHandler: Array<(user?: User) => void> = [];
    private readonly timetableRequest: TimetableRequest;
    private readonly userRequest: UserRequest;
    private readonly eventRequest: EventRequest;
    private readonly classRequest: ClassRequest;
    private readonly memberRequest: MemberRequest;
    private readonly discordRequest: DiscordRequest
    private readonly axios: Axios;
    private refreshToken?: string;
    private currentUserID?: string;

    public constructor() {
        this.axios = Axios.getInstance();
        this.timetableRequest = new TimetableRequest();
        this.userRequest = new UserRequest();
        this.eventRequest = new EventRequest();
        this.classRequest = new ClassRequest();
        this.memberRequest = new MemberRequest();
        this.discordRequest = new DiscordRequest();
        const refreshToken = localStorage.getItem('refresh-token')
        if (refreshToken) {
            this.refreshToken = refreshToken;
            axios.get('http://localhost:8080/api/token', {
                headers: {
                    Authorization: refreshToken
                }
            }).then(val => {
                this.setToken(val.headers);
                this.getCurrentUser().then(value => {
                    this.currentUserID = value.id;
                    this.triggerOnAuthStateChange(value)
                })
            })
        }
    }

    public async getCurrentUser(): Promise<User> {
        return await this.userRequest.getCurrentUser();
    }

    public async logout(): Promise<void> {
        localStorage.removeItem('refresh-token')
        this.triggerOnAuthStateChange(undefined);
    }

    public async login(email: string, password: string): Promise<void> {
        const loginResponse = await this.userRequest.login(email, password);
        this.currentUserID = loginResponse.data.userid;

        this.setToken(loginResponse.headers);
        this.updateToken(loginResponse.data.expires);
        localStorage.setItem('refresh-token', loginResponse.headers['refresh-token'])
        this.triggerOnAuthStateChange(await this.getCurrentUser());
    }

    public async createAccount(user: PostUser): Promise<void> {
        const response = await this.userRequest.createAccount(user);
        this.setToken(response.headers);
        this.updateToken(response.data.expires);
        const newUser = response.data.user;
        this.currentUserID = response.data.user.id;
        this.triggerOnAuthStateChange(newUser);
    }

    public isAdmin(currentClass: Class): boolean {
        const role = currentClass.members.filter(val => this.currentUserID === val.user)[0]?.role;
        return role === 'owner' || role === 'admin'
    }

    public async getClass(id: string): Promise<Class> {
        return await this.classRequest.getClass(id);
    }

    public async createClass(name: string, description: string): Promise<void> {
        return await this.classRequest.createClass(name, description)
    }

    public getMember(memberList: Array<Member>, userID: string): Member | undefined {
        return memberList.filter(val => val.user === userID)[0];
    }

    public onUserChange(handler: (user?: User) => void) {
        this.onUserChangeHandler.push(handler)
    }

    public async getClasses(): Promise<Array<Class> | undefined> {
        return await this.classRequest.getClasses();
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
        return await this.eventRequest.getCalendar(classId)
    }

    public async createEvent(classID: string, event: Event): Promise<void> {
        await this.eventRequest.createEvent(classID, event)
    }

    public async getPendingMembers(classId: string): Promise<Array<User>> {
        return await this.memberRequest.getPendingMembers(classId);
    }


    public async replyToRequest(classID: string, userID: string, accept: boolean): Promise<void> {
        await this.memberRequest.replyToRequest(classID, userID, accept);
    }

    public async changeEmail(email: string) {
        return this.userRequest.changeEmail(email);
    }

    public async changeDescription(description: string) {
        return this.userRequest.changeDescription(description)
    }

    public async changePassword(password: string, oldPassword: string) {
        return this.userRequest.changePassword(password, oldPassword)
    }

    public async deleteClassMember(classId: string, memberId: string): Promise<void> {
        await this.memberRequest.deleteClassMember(classId, memberId)
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
                console.log('Requesting new Token')
                const response = await axios.get('http://localhost:8080/api/token', {
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
        if(header['refresh-token'])
            this.refreshToken = header['refresh-token']
        this.axios.setAxios(header['token']);
    }
}
