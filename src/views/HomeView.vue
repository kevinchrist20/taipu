<script setup>
import { invoke } from '../types/tauri_commands';

function exitApp() {
    invoke('exit_app').then(() => {
        console.log('App exited');
    });
}

</script>

<template>
    <div class="h-screen w-screen bg-gray-800 text-white flex items-center justify-center font-mono">
      <div class="fade-in p-8 rounded-lg bg-gray-900 bg-opacity-80 shadow-lg max-w-lg w-full">
        <h1 class="text-4xl font-bold text-center mb-6">Welcome to Taipu</h1>
        <div class="grid grid-cols-1 gap-4">
          <button
            class="transition-transform transform hover:scale-105 bg-gray-700 hover:bg-gray-600 text-white font-semibold rounded-lg py-3 px-5 shadow-md"
          >
            Continue Lesson
          </button>
          <button
            class="transition-transform transform hover:scale-105 bg-gray-700 hover:bg-gray-600 text-white font-semibold rounded-lg py-3 px-5 shadow-md"
          >
            Start New Lesson
          </button>
          <button
            class="transition-transform transform hover:scale-105 bg-gray-700 hover:bg-gray-600 text-white font-semibold rounded-lg py-3 px-5 shadow-md"
          >
            View Progress
          </button>
          <button
            class="transition-transform transform hover:scale-105 bg-gray-700 hover:bg-gray-600 text-white font-semibold rounded-lg py-3 px-5 shadow-md"
          >
            Settings
          </button>
          <button
            class="transition-transform transform hover:scale-105 bg-red-600 hover:bg-red-700 text-white font-semibold rounded-lg py-3 px-5 shadow-md"
            @click="exitApp"
          >
            Quit Taipu
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