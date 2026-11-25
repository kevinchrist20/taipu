<script setup lang="ts">
import { computed, ref } from 'vue';
import type { DailyActivityStat } from '../types/bindings/DailyActivityStat';
import { TrendingUp, Calendar, Zap, Target } from 'lucide-vue-next';

interface Props {
  dailyTrends: DailyActivityStat[];
}

const props = defineProps<Props>();

type TimeRange = '7d' | '30d' | 'all';
const activeRange = ref<TimeRange>('30d');

// Filter data by selected range
const filteredData = computed(() => {
  if (!props.dailyTrends || props.dailyTrends.length === 0) {
    return [];
  }

  const sorted = [...props.dailyTrends].sort((a, b) => a.date.localeCompare(b.date));

  if (activeRange.value === '7d') {
    return sorted.slice(-7);
  } else if (activeRange.value === '30d') {
    return sorted.slice(-30);
  }
  return sorted;
});

// Chart dimensions inside SVG viewBox
const width = 640;
const height = 220;
const padding = { top: 25, right: 30, bottom: 35, left: 45 };

const graphWidth = computed(() => width - padding.left - padding.right);
const graphHeight = computed(() => height - padding.top - padding.bottom);

// Min and max scales
const maxWpm = computed(() => {
  if (filteredData.value.length === 0) return 60;
  const max = Math.max(...filteredData.value.map(d => d.avgWpm));
  return Math.ceil(Math.max(max * 1.15, 40));
});

const minWpm = computed(() => {
  if (filteredData.value.length === 0) return 0;
  const min = Math.min(...filteredData.value.map(d => d.avgWpm));
  return Math.max(0, Math.floor(min * 0.8));
});

// Coordinate mappings
interface Point {
  x: number;
  yWpm: number;
  yAcc: number;
  data: DailyActivityStat;
}

const points = computed<Point[]>(() => {
  const data = filteredData.value;
  if (data.length === 0) return [];
  if (data.length === 1) {
    return [
      {
        x: padding.left + graphWidth.value / 2,
        yWpm: getYForWpm(data[0].avgWpm),
        yAcc: getYForAccuracy(data[0].avgAccuracy),
        data: data[0]
      }
    ];
  }

  return data.map((d, index) => {
    const x = padding.left + (index / (data.length - 1)) * graphWidth.value;
    const yWpm = getYForWpm(d.avgWpm);
    const yAcc = getYForAccuracy(d.avgAccuracy);
    return { x, yWpm, yAcc, data: d };
  });
});

function getYForWpm(wpm: number): number {
  const range = maxWpm.value - minWpm.value || 1;
  const normalized = (wpm - minWpm.value) / range;
  return padding.top + (1 - normalized) * graphHeight.value;
}

function getYForAccuracy(acc: number): number {
  // Accuracy is bounded between 50% and 100%
  const minAcc = 60;
  const maxAcc = 100;
  const clamped = Math.max(minAcc, Math.min(maxAcc, acc));
  const normalized = (clamped - minAcc) / (maxAcc - minAcc);
  return padding.top + (1 - normalized) * graphHeight.value;
}

// Generate smooth bezier path for WPM
const wpmAreaPath = computed(() => {
  if (points.value.length === 0) return '';
  if (points.value.length === 1) {
    const p = points.value[0];
    const bottom = padding.top + graphHeight.value;
    return `M ${p.x - 20} ${bottom} L ${p.x - 20} ${p.yWpm} L ${p.x + 20} ${p.yWpm} L ${p.x + 20} ${bottom} Z`;
  }

  let line = `M ${points.value[0].x} ${points.value[0].yWpm}`;
  for (let i = 0; i < points.value.length - 1; i++) {
    const p0 = points.value[i];
    const p1 = points.value[i + 1];
    const cpX = (p0.x + p1.x) / 2;
    line += ` C ${cpX} ${p0.yWpm}, ${cpX} ${p1.yWpm}, ${p1.x} ${p1.yWpm}`;
  }

  const first = points.value[0];
  const last = points.value[points.value.length - 1];
  const bottom = padding.top + graphHeight.value;

  return `${line} L ${last.x} ${bottom} L ${first.x} ${bottom} Z`;
});

const wpmLinePath = computed(() => {
  if (points.value.length === 0) return '';
  if (points.value.length === 1) {
    const p = points.value[0];
    return `M ${p.x - 20} ${p.yWpm} L ${p.x + 20} ${p.yWpm}`;
  }

  let line = `M ${points.value[0].x} ${points.value[0].yWpm}`;
  for (let i = 0; i < points.value.length - 1; i++) {
    const p0 = points.value[i];
    const p1 = points.value[i + 1];
    const cpX = (p0.x + p1.x) / 2;
    line += ` C ${cpX} ${p0.yWpm}, ${cpX} ${p1.yWpm}, ${p1.x} ${p1.yWpm}`;
  }
  return line;
});

const accuracyLinePath = computed(() => {
  if (points.value.length <= 1) return '';
  let line = `M ${points.value[0].x} ${points.value[0].yAcc}`;
  for (let i = 0; i < points.value.length - 1; i++) {
    const p0 = points.value[i];
    const p1 = points.value[i + 1];
    const cpX = (p0.x + p1.x) / 2;
    line += ` C ${cpX} ${p0.yAcc}, ${cpX} ${p1.yAcc}, ${p1.x} ${p1.yAcc}`;
  }
  return line;
});

// Interactive hover crosshair
const hoveredIndex = ref<number | null>(null);

const hoveredPoint = computed(() => {
  if (hoveredIndex.value === null || hoveredIndex.value >= points.value.length) {
    return null;
  }
  return points.value[hoveredIndex.value];
});

function handleMouseMove(event: MouseEvent) {
  if (points.value.length === 0) return;
  const svg = (event.currentTarget as HTMLElement).querySelector('svg');
  if (!svg) return;

  const rect = svg.getBoundingClientRect();
  const mouseX = ((event.clientX - rect.left) / rect.width) * width;

  // Find closest point along x axis
  let closestIdx = 0;
  let minDiff = Infinity;

  points.value.forEach((pt, idx) => {
    const diff = Math.abs(pt.x - mouseX);
    if (diff < minDiff) {
      minDiff = diff;
      closestIdx = idx;
    }
  });

  hoveredIndex.value = closestIdx;
}

function handleMouseLeave() {
  hoveredIndex.value = null;
}

function formatDateLabel(dateStr: string): string {
  try {
    const parts = dateStr.split('-');
    if (parts.length === 3) {
      const date = new Date(Number(parts[0]), Number(parts[1]) - 1, Number(parts[2]));
      return date.toLocaleDateString(undefined, { month: 'short', day: 'numeric' });
    }
    return dateStr;
  } catch {
    return dateStr;
  }
}
</script>

<template>
  <div class="rounded-2xl border border-border bg-card p-6 shadow-sm">
    <!-- Header: Title, Metric Legend & Range Switcher -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-border/60">
      <div>
        <div class="flex items-center gap-2">
          <TrendingUp class="h-4 w-4 text-primary" />
          <h3 class="text-xl font-semibold font-display text-foreground">Speed & Accuracy Trend</h3>
        </div>
        <p class="text-xs text-muted-foreground mt-0.5">Track your daily typing trajectory over time</p>
      </div>

      <div class="flex flex-wrap items-center gap-4">
        <!-- Legend -->
        <div class="flex items-center gap-3 text-xs">
          <span class="flex items-center gap-1.5 text-foreground font-medium">
            <span class="h-2.5 w-2.5 rounded-full bg-primary" />
            Speed (WPM)
          </span>
          <span class="flex items-center gap-1.5 text-muted-foreground">
            <span class="h-2 w-2 rounded-full bg-amber-400" />
            Accuracy (%)
          </span>
        </div>

        <!-- Range Buttons -->
        <div class="flex items-center rounded-lg border border-border bg-surface p-0.5 text-xs font-medium">
          <button
            type="button"
            class="px-2.5 py-1 rounded-lg transition-colors"
            :class="[activeRange === '7d' ? 'bg-primary text-primary-foreground font-semibold shadow-sm' : 'bg-surface/80 text-muted-foreground/70 hover:text-foreground hover:bg-surface']"
            @click="activeRange = '7d'"
          >
            7D
          </button>
          <button
            type="button"
            class="px-2.5 py-1 rounded-lg transition-colors"
            :class="[activeRange === '30d' ? 'bg-primary text-primary-foreground font-semibold shadow-sm' : 'bg-surface/80 text-muted-foreground/70 hover:text-foreground hover:bg-surface']"
            @click="activeRange = '30d'"
          >
            30D
          </button>
          <button
            type="button"
            class="px-2.5 py-1 rounded-lg transition-colors"
            :class="[activeRange === 'all' ? 'bg-primary text-primary-foreground font-semibold shadow-sm' : 'bg-surface/80 text-muted-foreground/70 hover:text-foreground hover:bg-surface']"
            @click="activeRange = 'all'"
          >
            All Time
          </button>
        </div>
      </div>
    </div>

    <!-- Chart Container -->
    <div
      class="relative mt-4 w-full cursor-crosshair select-none"
      @mousemove="handleMouseMove"
      @mouseleave="handleMouseLeave"
    >
      <!-- Empty State -->
      <div
        v-if="points.length === 0"
        class="flex h-full flex-col items-center justify-center text-center p-6 text-muted-foreground"
      >
        <Calendar class="h-8 w-8 mb-2 opacity-40" />
        <p class="text-sm font-medium">No trend data logged yet</p>
        <p class="text-xs max-w-xs mt-1 text-muted-foreground/80">
          Complete typing lessons across multiple days to unlock your performance growth curve.
        </p>
      </div>

      <!-- Active SVG Chart -->
      <svg
        v-else
        :viewBox="`0 0 ${width} ${height}`"
        class="h-full w-full overflow-visible"
        preserveAspectRatio="none"
      >
        <defs>
          <linearGradient id="wpmAreaGradient" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stop-color="var(--primary)" stop-opacity="0.32" />
            <stop offset="85%" stop-color="var(--primary)" stop-opacity="0.02" />
            <stop offset="100%" stop-color="var(--primary)" stop-opacity="0" />
          </linearGradient>
        </defs>

        <!-- Horizontal Guide Gridlines -->
        <g stroke="currentColor" stroke-opacity="0.08" stroke-dasharray="3 3">
          <line
            :x1="padding.left"
            :y1="padding.top"
            :x2="width - padding.right"
            :y2="padding.top"
          />
          <line
            :x1="padding.left"
            :y1="padding.top + graphHeight / 2"
            :x2="width - padding.right"
            :y2="padding.top + graphHeight / 2"
          />
          <line
            :x1="padding.left"
            :y1="padding.top + graphHeight"
            :x2="width - padding.right"
            :y2="padding.top + graphHeight"
          />
        </g>

        <!-- Left Y-Axis Labels (WPM) -->
        <text
          :x="padding.left - 8"
          :y="padding.top + 4"
          text-anchor="end"
          class="fill-muted-foreground text-[10px] font-mono"
        >
          {{ maxWpm }}
        </text>
        <text
          :x="padding.left - 8"
          :y="padding.top + graphHeight / 2 + 3"
          text-anchor="end"
          class="fill-muted-foreground text-[10px] font-mono"
        >
          {{ Math.round((maxWpm + minWpm) / 2) }}
        </text>
        <text
          :x="padding.left - 8"
          :y="padding.top + graphHeight + 3"
          text-anchor="end"
          class="fill-muted-foreground text-[10px] font-mono"
        >
          {{ minWpm }}
        </text>

        <!-- WPM Area Fill -->
        <path :d="wpmAreaPath" fill="url(#wpmAreaGradient)" />

        <!-- WPM Curve Line -->
        <path
          :d="wpmLinePath"
          fill="none"
          stroke="var(--primary)"
          stroke-width="2.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        />

        <!-- Accuracy Line (Secondary) -->
        <path
          v-if="accuracyLinePath"
          :d="accuracyLinePath"
          fill="none"
          stroke="#f59e0b"
          stroke-width="1.8"
          stroke-dasharray="4 3"
          stroke-linecap="round"
        />

        <!-- Hover Guideline & Data Indicator -->
        <g v-if="hoveredPoint">
          <line
            :x1="hoveredPoint.x"
            :y1="padding.top"
            :x2="hoveredPoint.x"
            :y2="padding.top + graphHeight"
            stroke="var(--foreground)"
            stroke-opacity="0.25"
            stroke-width="1"
            stroke-dasharray="2 2"
          />

          <!-- Point Dot for WPM -->
          <circle
            :cx="hoveredPoint.x"
            :cy="hoveredPoint.yWpm"
            r="5"
            fill="var(--card)"
            stroke="var(--primary)"
            stroke-width="2.5"
          />

          <!-- Point Dot for Accuracy -->
          <circle
            :cx="hoveredPoint.x"
            :cy="hoveredPoint.yAcc"
            r="3.5"
            fill="var(--card)"
            stroke="#f59e0b"
            stroke-width="2"
          />
        </g>

        <!-- Date Tick Labels (First, Middle, Last) -->
        <g v-if="points.length > 0" class="fill-muted-foreground text-[10px] font-mono">
          <text :x="points[0].x" :y="height - 10" text-anchor="start">
            {{ formatDateLabel(points[0].data.date) }}
          </text>
          <text
            v-if="points.length > 2"
            :x="points[Math.floor(points.length / 2)].x"
            :y="height - 10"
            text-anchor="middle"
          >
            {{ formatDateLabel(points[Math.floor(points.length / 2)].data.date) }}
          </text>
          <text
            v-if="points.length > 1"
            :x="points[points.length - 1].x"
            :y="height - 10"
            text-anchor="end"
          >
            {{ formatDateLabel(points[points.length - 1].data.date) }}
          </text>
        </g>
      </svg>

      <!-- Floating Hover Tooltip -->
      <div
        v-if="hoveredPoint"
        class="pointer-events-none absolute top-2 rounded-xl border border-border bg-surface-elevated/95 px-3 py-2 shadow-lg backdrop-blur-md transition-all duration-150 text-xs"
        :style="{
          left: `${Math.min(width - 160, Math.max(20, (hoveredPoint.x / width) * 100))}%`,
          transform: 'translateX(-50%)'
        }"
      >
        <p class="font-medium text-muted-foreground text-[11px] mb-1">
          {{ formatDateLabel(hoveredPoint.data.date) }} ({{ hoveredPoint.data.completionsCount }} test{{ hoveredPoint.data.completionsCount === 1 ? '' : 's' }})
        </p>
        <div class="flex items-center gap-3">
          <span class="flex items-center gap-1 font-mono font-bold text-primary">
            <Zap class="h-3 w-3" />
            {{ Math.round(hoveredPoint.data.avgWpm) }} WPM
          </span>
          <span class="flex items-center gap-1 font-mono font-semibold text-amber-500">
            <Target class="h-3 w-3" />
            {{ Math.round(hoveredPoint.data.avgAccuracy) }}%
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
