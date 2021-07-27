import Axios from './AxiosInstance';
import User from "../data/user/User";

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

    public async deleteClassMember(classId: string, memberId: string): Promise<void> {
        await this.axios.axios.delete(`/classes/${classId}/members/${memberId}`)
    }

    public async requestToJoinClass(classId: string) {
        await this.axios.axios.post(`/classes/${classId}`)
    }

    public async getPendingMembers(classId: string): Promise<Array<User>> {
        return await this.axios.axios.get<Array<User>>(`/classes/${classId}/requests`).then(r => r.data);
    }
}