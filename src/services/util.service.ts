import { invoke } from '../types/tauri_commands';

export default {
    async exitApp() {
        try {
            await invoke('exit_app', undefined);
        } catch (error) {
            console.error(error);
        }
    },
}