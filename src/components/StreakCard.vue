<script setup lang="ts">
import { computed } from 'vue';
import type { StreakInfo } from '../types/bindings/StreakInfo';
import type { DailyActivityStat } from '../types/bindings/DailyActivityStat';
import { Flame, Trophy, CalendarCheck } from 'lucide-vue-next';

interface Props {
  streakInfo: StreakInfo;
  dailyTrends: DailyActivityStat[];
}

const props = defineProps<Props>();

// Generate the past 28 days (4 weeks) for the punchcard grid
interface DaySlot {
  dateStr: string;
  dayOfMonth: number;
  active: boolean;
  completions: number;
  isToday: boolean;
}

const recentDays = computed<DaySlot[]>(() => {
  const activeDatesMap = new Map<string, number>();
  (props.dailyTrends || []).forEach(d => {
    activeDatesMap.set(d.date, d.completionsCount);
  });

  const slots: DaySlot[] = [];
  const today = new Date();
  const todayStr = `${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}-${String(today.getDate()).padStart(2, '0')}`;

  for (let i = 27; i >= 0; i--) {
    const d = new Date();
    d.setDate(today.getDate() - i);
    const dateStr = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
    const completions = activeDatesMap.get(dateStr) || 0;

    slots.push({
      dateStr,
      dayOfMonth: d.getDate(),
      active: completions > 0,
      completions,
      isToday: dateStr === todayStr,
    });
  }

  return slots;
});

const isTrainedToday = computed(() => {
  const todaySlot = recentDays.value.find(d => d.isToday);
  return Boolean(todaySlot?.active);
});
</script>

<template>
  <div class="rounded-2xl border border-border bg-card p-6 shadow-sm flex flex-col justify-between">
    <!-- Header -->
    <div class="flex items-start justify-between">
      <div>
        <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground font-medium">Daily Consistency</p>
        <h3 class="text-2xl font-bold font-display text-foreground mt-1 flex items-center gap-2">
          <span class="font-mono text-3xl font-bold" :class="props.streakInfo.currentStreak > 0 ? 'text-amber-500' : 'text-muted-foreground'">
            {{ props.streakInfo.currentStreak }}
          </span>
          <span class="text-base font-medium text-foreground">day streak</span>
          <Flame
            class="h-6 w-6 transition-transform"
            :class="props.streakInfo.currentStreak > 0 ? 'text-amber-500 fill-amber-500 animate-pulse' : 'text-muted-foreground opacity-40'"
          />
        </h3>
      </div>

      <!-- Best Streak Pill -->
      <div class="flex items-center gap-1.5 rounded-full border border-border bg-surface px-2.5 py-1 text-xs">
        <Trophy class="h-3.5 w-3.5 text-amber-400" />
        <span class="text-muted-foreground">Best:</span>
        <span class="font-mono font-semibold text-foreground">{{ props.streakInfo.bestStreak }}d</span>
      </div>
    </div>

    <!-- Status Message -->
    <div class="my-4 rounded-xl border border-border/80 bg-surface/50 p-3 text-xs">
      <div class="flex items-center gap-2">
        <CalendarCheck class="h-4 w-4 text-primary shrink-0" />
        <p v-if="isTrainedToday" class="text-foreground font-medium">
          You've practiced today! Streak is safely secured.
        </p>
        <p v-else-if="props.streakInfo.currentStreak > 0" class="text-amber-500 font-medium">
          Practice today to extend your {{ props.streakInfo.currentStreak }}-day streak!
        </p>
        <p v-else class="text-muted-foreground">
          Complete a lesson today to ignite a new practice streak.
        </p>
      </div>
    </div>

    <!-- Activity Punchcard Grid (28 days) -->
    <div>
      <div class="flex items-center justify-between text-[11px] text-muted-foreground mb-2">
        <span>Last 4 weeks</span>
        <span class="flex items-center gap-2">
          <span class="flex items-center gap-1">
            <span class="h-2 w-2 rounded-sm bg-surface border border-border" /> Missed
          </span>
          <span class="flex items-center gap-1">
            <span class="h-2 w-2 rounded-sm bg-primary" /> Active
          </span>
        </span>
      </div>

      <div class="grid grid-cols-7 gap-1.5">
        <div
          v-for="slot in recentDays"
          :key="slot.dateStr"
          class="group relative flex h-7 w-full items-center justify-center rounded-md text-[10px] font-mono transition-all"
          :class="[
            slot.active
              ? 'bg-primary text-primary-foreground font-semibold shadow-xs'
              : 'bg-surface/80 text-muted-foreground/70 hover:bg-surface border border-border/40',
            slot.isToday ? 'ring-2 ring-primary ring-offset-1 ring-offset-card' : ''
          ]"
        >
          <span>{{ slot.dayOfMonth }}</span>

          <!-- Tooltip on hover -->
          <div class="pointer-events-none absolute bottom-full mb-1.5 hidden rounded-md border border-border bg-surface-elevated px-2 py-1 text-[10px] text-foreground shadow-md whitespace-nowrap group-hover:block z-20">
            <p class="font-semibold">{{ slot.dateStr }}</p>
            <p class="text-muted-foreground">{{ slot.active ? `${slot.completions} completed` : 'No sessions' }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
