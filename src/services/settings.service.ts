import { load } from '@tauri-apps/plugin-store';

const STORE_FILE = 'settings.json';
const THEME_KEY = 'theme';
const DEFAULT_THEME = 'LIGHT';

let storePromise: ReturnType<typeof load> | null = null;

function normalizeTheme(theme: string): string {
    const value = theme.trim().toUpperCase();
    return value === 'DARK' ? 'DARK' : 'LIGHT';
}

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
