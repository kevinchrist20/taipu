import { User } from '../types/bindings';
import { invoke } from '../types/tauri_commands';

export default {
    async getAllUsers(): Promise<User[]> {
        try {
            return await invoke('get_all_users');
        } catch (error) {
            console.error(error);
            return [];
        }
    },
}