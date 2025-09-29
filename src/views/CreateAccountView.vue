<script setup lang="ts">
import { ref } from 'vue';
import { difficultyOptions, keyboardOptions, languageOptions, themeOptions } from '../types';
import UserService from '../services/user.service';
import useAlert from '../composables/useAlert';
import router from '../router';
import BackButton from '../components/BackButton.vue';

const name = ref('');
const username = ref('');
const keyboardType = ref('QWERTY');
const language = ref('ENGLISH');
const theme = ref('LIGHT');
const lessonDifficulty = ref('BEGINNER');

const isLoading = ref(false);

function createAccount() {
    if (!name.value || !username.value) {
        useAlert().setAlert({ message: 'Name and username are required', type: 'danger' });
        return;
    }

    isLoading.value = true;

    UserService.createUser({
        name: name.value?.toLowerCase(),
        username: username.value?.toLowerCase(),
        keyboardType: keyboardType.value,
        language: language.value,
        theme: theme.value,
        lessonDifficulty: lessonDifficulty.value,
    }).then(() => {
        useAlert().setAlert({ message: 'Account created successfully', type: 'success' });
        router.back();
    }).catch((error) => {
        useAlert().setAlert({ message: error, type: 'danger' })
    }).finally(() => {
        isLoading.value = false;
    });
}

</script>

<template>
    <div class="flex flex-col items-center justify-center min-h-screen bg-gray-800 text-white font-mono px-6 relative">
        <!-- Back Button -->
        <BackButton />

        <div class="bg-gray-900 rounded-lg shadow-lg p-8 w-full max-w-lg">
            <h1 class="text-3xl font-bold text-center mb-6">Create Account</h1>
            <form @submit.prevent="createAccount">
                <!-- Name -->
                <div class="mb-4">
                    <label for="name" class="block text-sm font-semibold mb-2">Full Name</label>
                    <input id="name" type="text" v-model="name" placeholder="Enter your full name"
                        class="w-full p-3 rounded-md bg-gray-700 text-white border border-gray-600 focus:outline-none focus:ring-2 focus:ring-indigo-500" />
                </div>

                <!-- Username -->
                <div class="mb-4">
                    <label for="username" class="block text-sm font-semibold mb-2">Username</label>
                    <input id="username" type="text" v-model="username" placeholder="Choose a username"
                        class="w-full p-3 rounded-md bg-gray-700 text-white border border-gray-600 focus:outline-none focus:ring-2 focus:ring-indigo-500" />
                </div>

                <!-- Keyboard Type -->
                <div class="mb-4">
                    <label for="keyboardType" class="block text-sm font-semibold mb-2">Keyboard Type</label>
                    <select id="keyboardType" v-model="keyboardType"
                        class="w-full p-3 rounded-md bg-gray-700 text-white border border-gray-600 focus:outline-none focus:ring-2 focus:ring-indigo-500">
                        <option v-for="option in keyboardOptions" :key="option.value" :value="option.value">
                            {{ option.label }}
                        </option>
                    </select>
                </div>

                <!-- Language -->
                <div class="mb-4">
                    <label for="language" class="block text-sm font-semibold mb-2">Language</label>
                    <select id="language" v-model="language"
                        class="w-full p-3 rounded-md bg-gray-700 text-white border border-gray-600 focus:outline-none focus:ring-2 focus:ring-indigo-500">
                        <option v-for="option in languageOptions" :key="option.value" :value="option.value">
                            {{ option.label }}
                        </option>
                    </select>
                </div>

                <!-- Theme -->
                <div class="mb-4">
                    <label class="block text-sm font-semibold mb-2">Theme</label>
                    <div class="flex gap-4">
                        <label v-for="option in themeOptions" :key="option.value" class="flex items-center">
                            <input type="radio" v-model="theme" :value="option.value"
                                class="mr-2 text-indigo-500 focus:ring-indigo-500" />
                            {{ option.label }}
                        </label>
                    </div>
                </div>

                <!-- Lesson Difficulty -->
                <div class="mb-6">
                    <label class="block text-sm font-semibold mb-2">Lesson Difficulty</label>
                    <div class="flex gap-4">
                        <label v-for="option in difficultyOptions" :key="option.value" class="flex items-center">
                            <input type="radio" v-model="lessonDifficulty" :value="option.value"
                                class="mr-2 text-indigo-500 focus:ring-indigo-500" />
                            {{ option.label }}
                        </label>
                    </div>
                </div>

                <!-- Submit Button -->
                <button type="submit"
                    class="w-full p-3 rounded-md bg-indigo-600 hover:bg-indigo-500 text-white font-semibold transition-transform transform hover:scale-105">
                    Create Account
                </button>
            </form>
        </div>
    </div>
</template>

<style scoped></style>