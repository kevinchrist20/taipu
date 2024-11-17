<script setup lang="ts">
  defineProps<{ rateInfo:{ timer: string, accuracy:number, percentComplete:number, wpm:number } }>()
</script>
<template>
  <div class="grid grid-cols-10 px-3 py-4 bg-zinc-1 dark:bg-zinc-7 rounded-t-lg text-zinc-5 dark:text-white">
    <div class="grid grid-cols-2 gap-2 col-span-10 md:col-span-5">
      <div>Timer: {{ rateInfo.timer }}</div>
      <div>Accuracy: {{ rateInfo.accuracy }}%</div>
    </div>
    <div class="grid grid-cols-2 grid-content-end gap-5 items-center">
      <div class="col-span-2 md:col-span-1">Percent Complete: {{ rateInfo.percentComplete }}%</div>
      <div class="col-span-2 md:col-span-1">WPM: {{ rateInfo.wpm }}</div>
    </div>
  </div>
</template>


<style scoped>

</style>
