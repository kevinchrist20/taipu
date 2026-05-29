<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
    strokeWidth?: number
    diameter?: number
    percentage: number
}>();

const strokeWidth = computed(() => props.strokeWidth ?? 4)
const diameter = computed(() => props.diameter ?? 80)
const radius = computed(() => (diameter.value - strokeWidth.value) / 2)
const viewBox = computed(() => `0 0 ${diameter.value} ${diameter.value}`)
const dashArray = computed(() => radius.value * Math.PI * 2)
const dashOffset = computed(() => dashArray.value - (dashArray.value * (props.percentage || 0)) / 100)
const statusMessage = computed(() => `${props.percentage}%`)

</script>
<template>
    <svg :width="diameter" :height="diameter" :viewBox="viewBox">
        <circle 
            class="fill-none stroke-border" 
            :cx="diameter / 2" 
            :cy="diameter / 2" 
            :r="radius"
            :stroke-width="strokeWidth" 
        />
        <circle 
            :cx="diameter / 2" 
            :cy="diameter / 2" 
            :r="radius"
            class="fill-none stroke-primary transition-all delay-200 ease-in" 
            :stroke-width="strokeWidth"
            stroke-linecap="round" 
            :stroke-dasharray="dashArray" 
            :stroke-dashoffset="dashOffset"
            :transform="`rotate(-90 ${diameter / 2} ${diameter / 2})`" 
        />
        <text 
            x="50%" 
            y="50%" 
            dy=".3em" 
            text-anchor="middle" 
            fill="currentColor"
            class="font-semibold text-xs"
        >
            {{ statusMessage }}
        </text>
    </svg>
</template>