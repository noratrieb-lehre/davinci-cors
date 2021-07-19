import axios, {AxiosInstance} from "axios";
import User from "../data/user/User";


export default class UserService {
    private axiosInstance: AxiosInstance;
    private currentUser: User | undefined;
    private onAuthStateChangeHandler: Array<(user: User | undefined) => void> = []

    public constructor() {
        this.axiosInstance = axios.create({});
    }

    public async logout(): Promise<void> {
        this.currentUser = undefined;
        this.onAuthStateChangeHandler.forEach(handler => handler(this.currentUser))
    }

    public onAuthStateChange(handler: (user: User | undefined) => void) {
        this.onAuthStateChangeHandler.push(handler)
    }

}
