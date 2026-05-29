import { invoke } from '../types/tauri_commands';

export default {
    async getAppTheme(): Promise<string> {
        return await invoke('get_app_theme');
    },

    async setAppTheme(theme: string): Promise<void> {
        await invoke('set_app_theme', { theme });
    }
};
