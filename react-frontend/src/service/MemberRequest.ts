import Axios from './AxiosInstance';

export default class MemberRequest {
    private readonly axios: Axios;

    public constructor() {
        this.axios = Axios.getInstance();
    }


    public async replyToRequest(classID: string, userID: string, accept: boolean): Promise<void> {
        await this.axios.axios.post(`/classes/${classID}/requests/${userID}`, {
            accept
        })
    }
}