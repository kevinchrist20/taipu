<script setup>
import { invoke } from '../types/tauri_commands';

function exitApp() {
    invoke('exit_app').then(() => {
        console.log('App exited');
    });
}

</script>

<template>
    <div class="h-screen w-screen bg-gradient-to-r from-purple-600 via-indigo-600 to-blue-600 text-white flex items-center justify-center font-mono">
      <div class="fade-in p-8 rounded-lg bg-gray-900 bg-opacity-80 shadow-lg max-w-lg w-full">
        <h1 class="text-4xl font-bold text-center mb-6 animate-pulse">Welcome to Taipu</h1>
        <div class="grid grid-cols-1 gap-4">
          <button
            class="transition-transform transform hover:scale-105 bg-gradient-to-r from-gray-700 to-gray-500 hover:from-gray-600 hover:to-gray-400 text-white font-semibold rounded-lg py-3 px-5 shadow-md"
          >
            Continue Lesson
          </button>
          <button
            class="transition-transform transform hover:scale-105 bg-gradient-to-r from-gray-700 to-gray-500 hover:from-gray-600 hover:to-gray-400 text-white font-semibold rounded-lg py-3 px-5 shadow-md"
          >
            Start New Lesson
          </button>
          <button
            class="transition-transform transform hover:scale-105 bg-gradient-to-r from-gray-700 to-gray-500 hover:from-gray-600 hover:to-gray-400 text-white font-semibold rounded-lg py-3 px-5 shadow-md"
          >
            View Progress
          </button>
          <button
            class="transition-transform transform hover:scale-105 bg-gradient-to-r from-gray-700 to-gray-500 hover:from-gray-600 hover:to-gray-400 text-white font-semibold rounded-lg py-3 px-5 shadow-md"
          >
            Settings
          </button>
          <button
            class="transition-transform transform hover:scale-105 bg-gradient-to-r from-red-600 to-red-500 hover:from-red-700 hover:to-red-600 text-white font-semibold rounded-lg py-3 px-5 shadow-md"
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
  
  .bg-gradient-to-r {
    background-size: 200% 200%;
    animation: gradientShift 6s ease infinite;
  }
  
  @keyframes gradientShift {
    0% {
      background-position: 0% 50%;
    }
    50% {
      background-position: 100% 50%;
    }
    100% {
      background-position: 0% 50%;
    }
  }
  </style>