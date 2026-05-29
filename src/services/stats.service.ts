import { UserStatistics } from '../types/bindings';
import { invoke } from '../types/tauri_commands';

export default {
    async getUserStatistics(userId: number, difficulty: string): Promise<UserStatistics> {
        return await invoke('get_user_statistics', {
            userId,
            difficulty: difficulty.toUpperCase()
        });
    }
};
