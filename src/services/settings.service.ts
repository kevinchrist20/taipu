import { load } from '@tauri-apps/plugin-store';
import { normalizeTheme } from '../utils/ui-formatters';

const STORE_FILE = 'settings.json';
const THEME_KEY = 'theme';
const DEFAULT_THEME = 'LIGHT';

let storePromise: ReturnType<typeof load> | null = null;

async function getStore() {
    if (!storePromise) {
        storePromise = load(STORE_FILE);
    }

    return await storePromise;
}

export default {
    async getAppTheme(): Promise<string> {
        const store = await getStore();
        const value = await store.get<string>(THEME_KEY);
        return value ? normalizeTheme(value) : DEFAULT_THEME;
    },

    async setAppTheme(theme: string): Promise<void> {
        const store = await getStore();
        await store.set(THEME_KEY, normalizeTheme(theme));
        await store.save();
    }
};
