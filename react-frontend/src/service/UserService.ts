import axios, {AxiosInstance} from "axios";
import User, {PostUser} from "../data/user/User";
import getClasses from "../mockup/ClassMockUp";
import Class from "../data/class/Class";
import Member from "../data/user/Member";
import MemberRole from "../data/user/MemberRole";
import getEvents from "../mockup/EventMockup";
import Event from "../data/event/Event";
import TimeTable from "../data/timetable/TimeTable";
import getTimeTable from "../mockup/TimeTableMockUp";


export default class UserService {
    private axiosInstance: AxiosInstance;
    private onUserChangeHandler: Array<(user: User | null) => void> = []

    public constructor() {
        this._currentUser = null;
        this.axiosInstance = axios.create({});
    }

    private _currentUser: User | null;

    get currentUser(): User | null {
        return this._currentUser;
    }

    public async logout(): Promise<void> {
        this._currentUser = null;
        this.triggerOnAuthStateChange();
    }

    public async login(email: string, password: string): Promise<void> {
        this._currentUser = {
            "id": 'hugo',
            'email': email,
            'description': 'Kein echter User'
        }
        this.triggerOnAuthStateChange();
    }

    public async createAccount(user: PostUser): Promise<void> {
        console.log(user)
    }

    public isAdmin(currentClass: Class): boolean {
        const role = currentClass.members.filter(val => this.currentUser?.id === val.user)[0]?.role;
        return role === 'owner' || role === 'admin'
    }

    public async getClass(id: string): Promise<Class> {
        return getClasses().filter(val => val.id === id)[0]
    }

    public getMember(memberList: Array<Member>, userID: string): Member | undefined {
        return memberList.filter(val => val.user === userID)[0];
    }

    public onUserChange(handler: (user: User | null) => void) {
        this.onUserChangeHandler.push(handler)
    }

    public async getClasses(): Promise<Array<Class>> {
        return getClasses();
    }

    public async getTimeTable(classId: string): Promise<TimeTable> {
        return getTimeTable();
    }

    public async getCalendar(classId: string): Promise<Array<Event>> {
        return getEvents();
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

    private triggerOnAuthStateChange() {
        this.onUserChangeHandler.forEach(handler => handler(this._currentUser))
    }
}
