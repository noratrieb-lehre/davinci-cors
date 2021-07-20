import axios, {AxiosInstance} from "axios";
import User from "../data/user/User";
import getClasses from "../mockup/ClassMockUp";


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

    public onAuthStateChange(handler: (user: User | null) => void) {
        this.onAuthStateChangeHandler.push(handler)
    }

    public getClasses(): Array<{ name: string, id: string }> {
        return getClasses().map(val => ({name: val.name, id: val.id}))
    }


    get currentUser(): User | null {
        return this._currentUser;
    }
}
