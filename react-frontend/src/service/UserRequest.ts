import {AxiosInstance, AxiosResponse} from "axios";
import User, {PostUser} from "../data/user/User";

export default class UserRequest {
    private axios: AxiosInstance;

    public constructor(axios: AxiosInstance) {
        this.axios = axios
    }

    public async login(email: string, password: string): Promise<AxiosResponse<{ userid: string, expires: number }>> {
        return await this.axios.post<{ userid: string, expires: number }>('/login', {
            email,
            password
        }).catch(err => console.error(err.response)) as AxiosResponse;
    }

    public async createAccount(user: PostUser): Promise<AxiosResponse> {
        console.log(user)
        return await this.axios.post<{ user: User, expires: number }>('/users', {
            password: user.password,
            email: user.email,
            description: user.description,
        }).catch(err => console.error(err.response)) as AxiosResponse
    }
}