<script setup lang="ts">
import UserService from '../services/user.service';
import UtilsService from '../services/util.service';

import { ref } from 'vue';
import { User } from '../types/bindings';
import router from '../router';

const users = ref<User[]>([]);
const homeScreenOptions = [
  {
    title: "Continue Lesson",
    disabled: users.value.length < 1,
    action: () => console.log("Continue Lesson")
  },
  {
    title: "Start New Lesson",
    action: () => router.push({ path: '/lesson-area' })
  },
  {
    title: "View Progress",
    action: () => console.log("View Progress")
  },
  {
    title: "Settings",
    action: () => console.log("Settings")
  },
  {
    title: "Quit Taipu",
    action: exitApp
  }
];

function exitApp() {
  UtilsService.exitApp();
}

async function getUsers() {
  users.value = await UserService.getAllUsers();
}

getUsers();
</script>

<template>
  <div class="h-screen w-screen bg-gray-800 text-white flex items-center justify-center font-mono">
    <div class="fade-in p-8 rounded-lg bg-gray-900 bg-opacity-80 shadow-lg max-w-lg w-full">
      <h1 class="text-4xl font-bold text-center mb-6">Welcome to Taipu</h1>
      <div class="grid grid-cols-1 gap-4">
        <button v-for="(option, index) in homeScreenOptions" :key="option.title" @click="option.action"
          :disabled="option.disabled" :class="[
            'font-semibold rounded-lg py-3 px-5 shadow-md',
            index === homeScreenOptions.length - 1
              ? 'bg-red-600 hover:bg-red-700 text-white'
              : 'bg-gray-700 text-white',
            {
              'opacity-50 pointer-events-none': option.disabled,
              'transition-transform transform hover:bg-gray-600 hover:scale-105': !option.disabled
            }
          ]">
          {{ option.title }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
@font-face {
  font-family: 'Retro';
  src: url('/fonts/retro.ttf') format('truetype');
}

.font-mono {
  font-family: 'Retro', monospace;
}

.fade-in {
  animation: fadeIn 1s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>