import Class from "../data/class/Class";
import Axios from './AxiosInstance'
import User from "../data/user/User";

export default class ClassRequest {
    private readonly axios: Axios;

    public constructor() {
        this.axios = Axios.getInstance();
    }

    public async getClass(id: string): Promise<Class> {
        return await this.axios.axios.get<Class>(`/classes/${id}`).then(r => r.data)
    }

    public async createClass(name: string, description: string): Promise<void> {
        await this.axios.axios.post('/classes', {
            name,
            description
        })
    }

    public async changeName(name: string, classId: string): Promise<void> {
        const response = await this.getClass(classId);
        await this.axios.axios.put(`/classes/${classId}`, {
            ...response,
            name
        })
    }

    public async changeDescription(description: string, classId: string): Promise<void> {
        const response = await this.getClass(classId);
        await this.axios.axios.put(`/classes/${classId}`, {
            ...response,
            description
        })
    }

    public async deleteClass(classId: string) {
        await this.axios.axios.delete(`/classes/${classId}`);
    }

    public async getClasses(): Promise<Array<Class> | undefined> {
        return this.axios.axios.get<User>('/users/me').then(r => r.data.classes);
    }
}