import axios, {AxiosInstance} from "axios";
import User, {PostUser} from "../data/user/User";
import getClasses from "../mockup/ClassMockUp";
import Class from "../data/class/Class";
import Member from "../data/user/Member";
import MemberRole from "../data/user/MemberRole";
import getEvents from "../mockup/EventMockup";
import Event from "../data/event/Event";


export default class UserService {
    private axiosInstance: AxiosInstance;
    private _currentUser: User | null;
    private onAuthStateChangeHandler: Array<(user: User | null) => void> = []

    public constructor() {
        this._currentUser = null;
        this.axiosInstance = axios.create({});
    }

    private triggerOnAuthStateChange() {
        this.onAuthStateChangeHandler.forEach(handler => handler(this._currentUser))
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

    public getClass(id: string): Class {
        return getClasses().filter(val => val.id === id)[0]
    }

    public getMember(memberList: Array<Member>, userID: string): Member | undefined {
        return memberList.filter(val => val.user === userID)[0];
    }

    public onAuthStateChange(handler: (user: User | null) => void) {
        this.onAuthStateChangeHandler.push(handler)
    }

    public getClasses(): Array<{ name: string, id: string }> {
        return getClasses().map(val => ({name: val.name, id: val.id}))
    }

    public getTimeTable(classId: string) {

    }

    public getCalendar(classId: string): Array<Event> {
        return getEvents();
    }


    get currentUser(): User | null {
        return this._currentUser;
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
}
