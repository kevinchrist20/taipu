<script setup lang="ts">
import { computed } from 'vue';

const { rateInfo } = defineProps<{ rateInfo:{ timer: string, accuracy:number, percentComplete:number, wpm:number } }>()
const timer = computed(() => {
  const [mins, secs] = rateInfo.timer.split(':');
  return { mins, secs }
})
</script>
<template>
  <div class="px-4 flex justify-between bg-zinc-1 dark:bg-zinc-7 rounded-t-lg text-zinc-5 items-center dark:text-white py-3">
    <div class="">
      <div class="flex items-center gap-2 text-3xl font-bold tracking-wide [&_div]:rounded-lg [&_div]:bg-zinc-3 dark:[&_div]:bg-zinc-9 [&_div]:p-1.5">
        <div>{{ timer.mins }}</div>
        <span>:</span>
        <div>{{ timer.secs }}</div>
      </div>
    </div>

 <div class="grid grid-cols-8 gap-5">
  <div class="col-span-4" :class="rateInfo.percentComplete === 0 && 'opacity-30'">
      <h1 class="font-semibold text-sm">Accuracy</h1>
      <div class="text-2xl font-semibold">{{ rateInfo.accuracy }}%</div>
    </div>

    <div class="col-span-4" :class="rateInfo.percentComplete === 0 && 'opacity-30'">
      <h1 class="font-semibold text-sm">WPM</h1>
      <div class="text-2xl font-semibold">{{ rateInfo.wpm }}</div>
    </div>
 </div>
  </div>
</template>


<style scoped>

</style>
