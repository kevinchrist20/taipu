<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { SessionStore } from '../storage';
import router from '../router';
import { routes } from '../constants';
import AppButton from '../components/AppButton.vue';
import { difficultyOptions, languageOptions, themeOptions } from '../types';
import useAlert from '../composables/useAlert';
import UserService from '../services/user.service';
import SettingsService from '../services/settings.service';

const saving = ref(false);
const loadingTheme = ref(false);
const language = ref('ENGLISH');
const lessonDifficulty = ref('BEGINNER');
const theme = ref('LIGHT');

const user = computed(() => SessionStore.user);

function applyTheme(themeValue: string) {
    const normalized = themeValue.toUpperCase();
    SessionStore.setTheme(normalized);
    document.documentElement.classList.toggle('dark', normalized === 'DARK');
}

async function loadSettings() {
    if (!user.value) {
        router.push({ path: routes.home });
        return;
    }

    language.value = user.value.language;
    lessonDifficulty.value = user.value.lessonDifficulty;

    loadingTheme.value = true;
    try {
        const storedTheme = await SettingsService.getAppTheme();
        theme.value = storedTheme;
        applyTheme(storedTheme);
    } catch (error) {
        console.error('Error loading app theme:', error);
        theme.value = 'LIGHT';
    } finally {
        loadingTheme.value = false;
    }
}

async function saveSettings() {
    if (!user.value) {
        router.push({ path: routes.home });
        return;
    }

    saving.value = true;

    try {
        const [updatedUser] = await Promise.all([
            UserService.updateUserPreferences(user.value.id, language.value, lessonDifficulty.value),
            SettingsService.setAppTheme(theme.value),
        ]);

        SessionStore.setUser(updatedUser);
        applyTheme(theme.value);

        useAlert().setAlert({
            type: 'success',
            message: 'Settings saved successfully.',
        });
    } catch (error) {
        console.error('Error saving settings:', error);
        useAlert().setAlert({
            type: 'danger',
            message: 'Unable to save settings. Please try again.',
        });
    } finally {
        saving.value = false;
    }
}

onMounted(loadSettings);
</script>

<template>
    <div class="min-h-full bg-background px-6 py-8">
        <div class="max-w-3xl mx-auto space-y-6">
            <div>
                <p class="text-xs tracking-[0.2em] uppercase text-muted-foreground font-medium">Preferences</p>
                <h1 class="text-4xl font-display font-bold text-foreground mt-2">Settings</h1>
                <p class="text-muted-foreground mt-2">Manage profile preferences and app appearance.</p>
            </div>

            <section class="rounded-2xl border border-border bg-card p-6 space-y-5">
                <div>
                    <h2 class="text-xl font-semibold text-foreground">Profile</h2>
                    <p class="text-sm text-muted-foreground mt-1">These settings affect lesson filtering and progression.</p>
                </div>

                <div class="space-y-4">
                    <div>
                        <p class="text-sm font-medium text-foreground mb-2">Language</p>
                        <div class="flex gap-2">
                            <button
                                v-for="option in languageOptions"
                                :key="option.value"
                                type="button"
                                @click="language = option.value"
                                :class="[
                                    'flex-1 py-2 rounded-xl text-sm font-medium border transition-all font-sans',
                                    language === option.value
                                        ? 'bg-primary text-primary-foreground border-primary'
                                        : 'bg-surface border-border text-muted-foreground hover:border-primary hover:text-foreground'
                                ]"
                            >
                                {{ option.label }}
                            </button>
                        </div>
                    </div>

                    <div>
                        <p class="text-sm font-medium text-foreground mb-2">Lesson Difficulty</p>
                        <div class="grid grid-cols-1 sm:grid-cols-3 gap-2">
                            <button
                                v-for="option in difficultyOptions"
                                :key="option.value"
                                type="button"
                                @click="lessonDifficulty = option.value"
                                :class="[
                                    'py-2 rounded-xl text-sm font-medium border transition-all font-sans',
                                    lessonDifficulty === option.value
                                        ? 'bg-primary text-primary-foreground border-primary'
                                        : 'bg-surface border-border text-muted-foreground hover:border-primary hover:text-foreground'
                                ]"
                            >
                                {{ option.label }}
                            </button>
                        </div>
                    </div>
                </div>
            </section>

            <section class="rounded-2xl border border-border bg-card p-6 space-y-5">
                <div>
                    <h2 class="text-xl font-semibold text-foreground">Appearance</h2>
                    <p class="text-sm text-muted-foreground mt-1">Theme is stored in settings.ini.</p>
                </div>

                <div>
                    <p class="text-sm font-medium text-foreground mb-2">Theme</p>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                        <button
                            v-for="option in themeOptions"
                            :key="option.value"
                            type="button"
                            @click="theme = option.value"
                            :disabled="loadingTheme"
                            :class="[
                                'py-2 rounded-xl text-sm font-medium border transition-all font-sans disabled:opacity-60 disabled:cursor-not-allowed',
                                theme === option.value
                                    ? 'bg-primary text-primary-foreground border-primary'
                                    : 'bg-surface border-border text-muted-foreground hover:border-primary hover:text-foreground'
                            ]"
                        >
                            {{ option.label }}
                        </button>
                    </div>
                </div>
            </section>

            <div class="flex items-center justify-end gap-3">
                <AppButton variant="ghost" @click="router.back()">Cancel</AppButton>
                <AppButton :loading="saving" @click="saveSettings">Save changes</AppButton>
            </div>
        </div>
    </div>
</template>
