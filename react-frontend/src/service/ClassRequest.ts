import {AxiosInstance} from "axios";
import Class from "../data/class/Class";

export default class ClassRequest {
    private axios: AxiosInstance;

    public constructor(axios: AxiosInstance) {
        this.axios = axios
    }

    public async getClass(id: string): Promise<Class> {
        return await this.axios.get<Class>(`/classes/${id}`).then(r => r.data)
    }
}