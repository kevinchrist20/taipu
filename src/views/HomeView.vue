<script>
import { exit } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api';

let message = invoke('greet');
console.log(message);


async function exitApp() {
    await exit(1)
}

</script>

<template>
    <div class="flex flex-col items-center justify-center h-screen bg-gray-800 text-white font-mono">
        <div class="text-4xl font-bold mb-8">Welcome to Taipu</div>
        <div class="grid grid-cols-1 gap-4 w-full max-w-md">
            <button class="bg-gray-600 hover:bg-gray-500 rounded-md py-2 px-4 transition-colors">
                Continue Lesson
            </button>

            <button class="bg-gray-600 hover:bg-gray-500 rounded-md py-2 px-4 transition-colors">
                Start New Lesson
            </button>

            <button class="bg-gray-600 hover:bg-gray-500 rounded-md py-2 px-4 transition-colors">
                View Progress
            </button>

            <button class="bg-gray-600 hover:bg-gray-500 rounded-md py-2 px-4 transition-colors">
                Settings
            </button>

            <button class="bg-gray-600 hover:bg-gray-500 rounded-md py-2 px-4 transition-colors" @click="exitApp">
                Quit Taipu
            </button>
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
</style>