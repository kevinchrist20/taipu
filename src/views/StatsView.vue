<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { SessionStore } from '../storage';
import router from '../router';
import { routes } from '../constants';
import StatsService from '../services/stats.service';
import type { UserStatistics } from '../types/bindings';

const loading = ref(false);
const errorMessage = ref('');

const stats = ref<UserStatistics>({
    avgWpm: 0,
    avgAccuracy: 0,
    bestWpm: 0,
    totalTimeSeconds: 0,
    totalCompletions: 0,
    totalLessonsCompleted: 0,
    gradeDistribution: [
        { grade: 'S', count: 0 },
        { grade: 'A', count: 0 },
        { grade: 'B', count: 0 },
        { grade: 'C', count: 0 },
        { grade: 'D', count: 0 }
    ],
    categoryProgress: [],
    recentActivity: []
});

const user = computed(() => SessionStore.user);
const totalGradeCount = computed(() =>
    stats.value.gradeDistribution.reduce((sum, item) => sum + item.count, 0)
);

const formattedTimeTyped = computed(() => {
    const totalSeconds = stats.value.totalTimeSeconds;
    if (totalSeconds < 60) {
        return `${totalSeconds}s`;
    }

    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    if (hours === 0) {
        return `${minutes}m`;
    }

    return `${hours}h ${minutes}m`;
});

const totalLessonsInTrack = computed(() =>
    stats.value.categoryProgress.reduce((sum, item) => sum + item.totalLessons, 0)
);

const totalCompletedInTrack = computed(() =>
    stats.value.categoryProgress.reduce((sum, item) => sum + item.completedLessons, 0)
);

function formatCategoryName(categoryName: string): string {
    return categoryName
        .split('-')
        .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
        .join(' ');
}

function formatAccuracy(value: number): string {
    return `${Math.round(value)}%`;
}

function formatCompletedAt(isoDate: string): string {
    const diffMs = Date.now() - new Date(isoDate).getTime();
    const diffMinutes = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMinutes / 60);
    const diffDays = Math.floor(diffHours / 24);

    if (diffMinutes < 1) return 'just now';
    if (diffMinutes < 60) return `${diffMinutes}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    return `${diffDays}d ago`;
}

function gradeColor(grade: string): string {
    switch (grade) {
        case 'S':
            return 'bg-indigo-500/20 text-indigo-300 border-indigo-500/30';
        case 'A':
            return 'bg-emerald-500/20 text-emerald-300 border-emerald-500/30';
        case 'B':
            return 'bg-amber-500/20 text-amber-300 border-amber-500/30';
        case 'C':
            return 'bg-slate-500/20 text-slate-300 border-slate-500/30';
        default:
            return 'bg-rose-500/20 text-rose-300 border-rose-500/30';
    }
}

function gradeBarWidth(count: number): number {
    if (!totalGradeCount.value) return 0;
    return (count / totalGradeCount.value) * 100;
}

async function loadStats() {
    if (!user.value) {
        router.push({ path: routes.home });
        return;
    }

    loading.value = true;
    errorMessage.value = '';

    try {
        stats.value = await StatsService.getUserStatistics(user.value.id, user.value.lessonDifficulty || 'BEGINNER');
    } catch (error) {
        console.error('Error loading user statistics:', error);
        errorMessage.value = 'Unable to load statistics right now.';
    } finally {
        loading.value = false;
    }
}

onMounted(loadStats);
</script>

<template>
    <div class="min-h-full bg-background px-6 py-8">
        <div class="max-w-7xl mx-auto">
            <div class="mb-8">
                <p class="text-xs tracking-[0.2em] uppercase text-muted-foreground font-medium">Performance</p>
                <h1 class="text-4xl font-display font-bold text-foreground mt-2">Your stats</h1>
                <p class="text-muted-foreground mt-2">Everything stored locally on this device. Train more to fill these in.</p>
            </div>

            <div v-if="loading" class="flex justify-center items-center h-56">
                <div class="w-10 h-10 rounded-full border-2 border-primary border-t-transparent animate-spin" />
            </div>

            <div v-else-if="errorMessage" class="rounded-2xl border border-destructive/30 bg-destructive/10 p-6 text-destructive">
                {{ errorMessage }}
            </div>

            <div v-else class="space-y-6">
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                    <div class="rounded-2xl border border-border bg-card p-5">
                        <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground mb-3">Avg WPM</p>
                        <p class="text-4xl font-bold font-mono text-primary">{{ Math.round(stats.avgWpm) }}</p>
                    </div>
                    <div class="rounded-2xl border border-border bg-card p-5">
                        <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground mb-3">Avg Accuracy</p>
                        <p class="text-4xl font-bold font-mono text-foreground">{{ formatAccuracy(stats.avgAccuracy) }}</p>
                    </div>
                    <div class="rounded-2xl border border-border bg-card p-5">
                        <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground mb-3">Best WPM</p>
                        <p class="text-4xl font-bold font-mono text-foreground">{{ Math.round(stats.bestWpm) }}</p>
                    </div>
                    <div class="rounded-2xl border border-border bg-card p-5">
                        <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground mb-3">Time Typed</p>
                        <p class="text-4xl font-bold font-mono text-foreground">{{ formattedTimeTyped }}</p>
                    </div>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-5">
                    <section class="lg:col-span-2 rounded-2xl border border-border bg-card p-5">
                        <h2 class="text-xl font-semibold text-foreground">Curriculum progress</h2>
                        <p class="text-muted-foreground text-sm mt-1 mb-4">
                            {{ totalCompletedInTrack }} of {{ totalLessonsInTrack }} lessons complete
                        </p>

                        <div class="space-y-4">
                            <div v-for="item in stats.categoryProgress" :key="item.category">
                                <div class="flex items-center justify-between text-sm mb-2">
                                    <p class="text-foreground font-medium">{{ formatCategoryName(item.category) }}</p>
                                    <p class="text-muted-foreground">{{ item.completedLessons }}/{{ item.totalLessons }}</p>
                                </div>
                                <div class="h-2 rounded-full bg-surface">
                                    <div class="h-2 rounded-full bg-primary transition-all duration-300"
                                        :style="{ width: `${item.totalLessons ? (item.completedLessons / item.totalLessons) * 100 : 0}%` }" />
                                </div>
                            </div>
                        </div>
                    </section>

                    <section class="rounded-2xl border border-border bg-card p-5">
                        <h2 class="text-xl font-semibold text-foreground">Grade distribution</h2>
                        <p class="text-muted-foreground text-sm mt-1 mb-4">Best per completion</p>

                        <div class="space-y-3">
                            <div v-for="item in stats.gradeDistribution" :key="item.grade" class="flex items-center gap-3">
                                <span class="w-7 h-7 rounded-md border text-xs font-bold flex items-center justify-center"
                                    :class="gradeColor(item.grade)">
                                    {{ item.grade }}
                                </span>
                                <div class="h-2 rounded-full bg-surface flex-1 overflow-hidden">
                                    <div class="h-2 rounded-full bg-primary/80" :style="{ width: `${gradeBarWidth(item.count)}%` }" />
                                </div>
                                <span class="text-muted-foreground text-sm w-5 text-right">{{ item.count }}</span>
                            </div>
                        </div>
                    </section>
                </div>

                <section class="rounded-2xl border border-border bg-card p-5">
                    <h2 class="text-xl font-semibold text-foreground mb-4">Recent activity</h2>

                    <div v-if="!stats.recentActivity.length" class="text-muted-foreground text-sm py-6">
                        No completions yet. Finish a lesson to populate your timeline.
                    </div>

                    <div v-else class="space-y-3">
                        <div v-for="item in stats.recentActivity" :key="item.id"
                            class="grid grid-cols-1 md:grid-cols-5 gap-3 md:gap-4 rounded-xl border border-border bg-surface-elevated p-4 items-center">
                            <div class="md:col-span-2">
                                <p class="text-foreground font-medium">{{ item.lessonTitle }}</p>
                                <p class="text-muted-foreground text-sm">{{ formatCategoryName(item.category) }}</p>
                            </div>
                            <p class="text-foreground text-sm font-mono">{{ Math.round(item.wpm) }} WPM</p>
                            <p class="text-foreground text-sm font-mono">{{ Math.round(item.accuracy) }}%</p>
                            <div class="flex items-center justify-between md:justify-end gap-3">
                                <span class="px-2.5 py-1 rounded-md text-xs border font-bold" :class="gradeColor(item.grade)">
                                    {{ item.grade }}
                                </span>
                                <span class="text-muted-foreground text-xs">{{ formatCompletedAt(item.completedAt) }}</span>
                            </div>
                        </div>
                    </div>
                </section>
            </div>
        </div>
    </div>
</template>