import Axios from "./AxiosInstance";

export default class DiscordRequest {
    private readonly axios: Axios;

    public constructor() {
        this.axios = Axios.getInstance();
    }

    public async linkClassToGuild(classId: string, snowflake: string) {
        await this.axios.axios.post(`/classes/${classId}/link`, {
            snowflake
        }).catch((err) => {
            throw new Error(err.response.data)
        })
    }

    public async linkAccountToDiscord(snowflake: string) {
        await this.axios.axios.post(`/users/me/link`, {
            snowflake
        }).catch((err) => {
            throw new Error(err.response.data)
        })
    }

}