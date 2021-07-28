import Axios from './AxiosInstance';
import Member from "../data/user/Member";
import Class from "../data/class/Class";

type ErrorMessage = 'success' | 'already-joined' | 'not-authorized' | 'class-not-found' | 'other-error';
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

    public async requestToJoinClass(classId: string): Promise<ErrorMessage> {
        let errMessage = undefined;
        await this.axios.axios.post(`/classes/${classId}/join`).catch((err) => {
            switch (err.response.status) {
                case 409:
                    errMessage = 'already-joined'
                    break
                case 401:
                    errMessage = 'not-authorized'
                    break
                case 404:
                    errMessage = 'class-not-found'
                    break;
                default:
                    errMessage = 'other-error'
            }
        })
        return errMessage || 'success'
    }

    public async getPendingMembers(classId: string): Promise<Array<Member>> {
        return await this.axios.axios.get<Array<Member>>(`/classes/${classId}/requests`).then(r => r.data);
    }

    public async getMembers(classId: string): Promise<Array<Member>> {
        return await this.axios.axios.get<Class>(`/classes/${classId}`).then(r => r.data.members);
    }

    public async updateClassMember(classId: string, member: Member): Promise<void> {
        return await this.axios.axios.put(`/classes/${classId}/members/${member.user}`, member)
    }
}

export type {ErrorMessage}