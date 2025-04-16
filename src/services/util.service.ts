import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { invoke } from '../types/tauri_commands';

export default {
    async exitApp() {
        try {
            await invoke('exit_app');
        } catch (error) {
            console.error(error);
        }
    },

    async checkForUpdates() {
        try {
            const update = await check();
            if (!update) {
                console.log('No updates found');
                return;
            }

            console.log(
                `Found update ${update?.version} from ${update?.date} with notes ${update?.body}`
            );

            let downloaded = 0;
            let contentLength = 0;

            if (update) {
                await update.downloadAndInstall((event) => {
                    switch (event.event) {
                        case 'Started':
                            contentLength = event.data.contentLength ?? 0;
                            console.log(`started downloading ${event.data.contentLength} bytes`);
                            break;
                        case 'Progress':
                            downloaded += event.data.chunkLength;
                            console.log(`downloaded ${downloaded} from ${contentLength}`);
                            break;
                        case 'Finished':
                            console.log('download finished');
                            break;
                    }
                });
            }

            console.log('Update installed');
            await relaunch();
        } catch (error) {
            console.error('Error checking for updates:', error);
        }
    }
}