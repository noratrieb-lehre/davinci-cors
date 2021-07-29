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
        }).catch((err) => {
            throw new Error(err.response.data)
        })
    }

    public async deleteClassMember(classId: string, memberId: string): Promise<void> {
        await this.axios.axios.delete(`/classes/${classId}/members/${memberId}`).catch((err) => {
            throw new Error(err.response.data)
        })
    }

    public async requestToJoinClass(classId: string): Promise<void> {
        await this.axios.axios.post(`/classes/${classId}/join`).catch((err) => {
            throw new Error(err.response.data)
        })
    }

    public async getPendingMembers(classId: string): Promise<Array<Member>> {
        return await this.axios.axios.get<Array<Member>>(`/classes/${classId}/requests`).then(r => r.data).catch((err) => {
            throw new Error(err.response.data)
        });
    }

    public async getBannedMembers(classId: string): Promise<Array<Member>> {
        return await this.axios.axios.get<Array<Member>>(`/classes/${classId}/bans`).then(r => r.data).catch((err) => {
            throw new Error(err.response.data)
        });
    }

    public async getMembers(classId: string): Promise<Array<Member>> {
        return await this.axios.axios.get<Class>(`/classes/${classId}`).then(r => r.data.members).catch((err) => {
            throw new Error(err.response.data)
        });
    }

    public async updateClassMember(classId: string, member: Member): Promise<void> {
        await this.axios.axios.put(`/classes/${classId}/members/${member.user}`, member).catch((err) => {
            throw new Error(err.response.data)
        })
        return;
    }

    public async updateDisplayName(classId: string, userId: string, displayName: string) {
        const member = await this.axios.axios.get<Member>(`/classes/${classId}/members/${userId}`).then(val => val.data);
        await this.axios.axios.put(`/classes/${classId}/members/${userId}`, {
            ...member,
            displayName
        })
    }
}

export type {ErrorMessage}