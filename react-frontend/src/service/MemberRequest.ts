import {AxiosInstance} from "axios";

export default class MemberRequest {
    private axios: AxiosInstance;

    public constructor(axios: AxiosInstance) {
        this.axios = axios
    }

    public async replyToRequest(classID: string, userID: string, accept: boolean): Promise<void> {
        await this.axios.post(`/classes/${classID}/requests/${userID}`, {
            accept
        })
    }
}