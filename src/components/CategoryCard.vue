<script setup lang="ts">
import { ChevronRight, Lock } from 'lucide-vue-next';

defineProps<{
  categoryName: string;
  difficulty: string;
  completed: number;
  total: number;
  isAvailable: boolean;
  description?: string;
}>();

const emit = defineEmits<{ click: [] }>();

type BadgeStyle = { color: string; bg: string; border: string };

function badgeStyle(d: string): BadgeStyle {
  const level = d.toLowerCase();
  if (level === 'beginner') return { color: 'var(--success)', bg: 'color-mix(in oklch, var(--success) 12%, transparent)', border: 'color-mix(in oklch, var(--success) 40%, transparent)' };
  if (level === 'intermediate') return { color: 'var(--warning)', bg: 'color-mix(in oklch, var(--warning) 12%, transparent)', border: 'color-mix(in oklch, var(--warning) 40%, transparent)' };
  return { color: 'var(--destructive)', bg: 'color-mix(in oklch, var(--destructive) 12%, transparent)', border: 'color-mix(in oklch, var(--destructive) 40%, transparent)' };
}
</script>

<template>
  <div class="bg-card rounded-2xl p-5 border border-border transition-all duration-200 group flex flex-col" :class="isAvailable
    ? 'cursor-pointer hover:border-primary hover:shadow-elevated'
    : 'opacity-50 cursor-not-allowed'" @click="emit('click')">
    <!-- Badge + Lesson Count -->
    <div class="flex items-center justify-between mb-3">
      <span class="text-xs font-semibold uppercase tracking-wider px-2.5 py-1 rounded-full border" :style="{
        color: badgeStyle(difficulty).color,
        backgroundColor: badgeStyle(difficulty).bg,
        borderColor: badgeStyle(difficulty).border,
      }">
        {{ difficulty }}
      </span>
      <span class="text-xs text-muted-foreground">{{ total }} lessons</span>
    </div>

    <!-- Category Name -->
    <h3 class="text-lg font-bold text-foreground mb-1.5">{{ categoryName }}</h3>

    <!-- Description -->
    <p v-if="description" class="text-sm text-muted-foreground leading-relaxed flex-1">{{ description }}</p>
    <div v-else class="flex-1" />

    <!-- Footer -->
    <div class="mt-4">
      <span v-if="isAvailable"
        class="text-sm font-medium text-primary flex items-center gap-0.5 group-hover:gap-1.5 transition-all">
        Start Category
        <ChevronRight :size="14" />
      </span>
      <span v-else class="text-xs text-muted-foreground flex items-center gap-1">
        <Lock :size="12" /> Locked
      </span>
    </div>
  </div>
</template>
