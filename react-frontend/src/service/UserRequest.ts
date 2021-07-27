import {AxiosResponse} from "axios";
import User, {PostUser} from "../data/user/User";
import Axios from './AxiosInstance'

export default class UserRequest {
    private readonly axios: Axios;

    public constructor() {
        this.axios = Axios.getInstance();
    }

    public async login(email: string, password: string): Promise<AxiosResponse<{ userid: string, expires: number }>> {
        return await this.axios.axios.post<{ userid: string, expires: number }>('/login', {
            email,
            password
        }).catch(err => console.error(err.response)) as AxiosResponse;
    }

    public async getCurrentUser(): Promise<User> {
        return await this.axios.axios.get<User>('/users/me').then(r => r.data);
    }

    public async createAccount(user: PostUser): Promise<AxiosResponse<{ user: User, expires: number }>> {
        return await this.axios.axios.post<{ user: User, expires: number }>('/users', {
            password: user.password,
            email: user.email,
            description: user.description,
        }).catch(err => console.error(err.response)) as AxiosResponse
    }

    public async changeEmail(email: string) {
        const response = this.axios.axios.get<User>('/users/me').then(r => r.data);
        await this.axios.axios.put('/users/me', {
            ...response,
            email
        })
    }

    public async changeDescription(description: string) {
        const response = this.axios.axios.get<User>('/users/me').then(r => r.data);
        await this.axios.axios.put('/users/me', {
            ...response,
            description
        })
    }

    public async deleteUser(): Promise<void> {
        await this.axios.axios.delete('/users/me');
    }

    public async changePassword(password: string, oldPassword: string) {
        await this.axios.axios.patch('/users/me/password', {
            password,
            oldPassword
        })
    }
}