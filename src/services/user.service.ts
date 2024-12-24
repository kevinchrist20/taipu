import { NewUser, User } from '../types/bindings';
import { invoke } from '../types/tauri_commands';

export default {
    async getUsers(): Promise<User[]> {
        const users = await invoke('get_all_users');
        return users;
    },

    async createUser(user: NewUser): Promise<void> {
        await invoke('add_user', { body: user });
    }
}