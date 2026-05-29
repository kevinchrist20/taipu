<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
    variant?: 'primary' | 'secondary' | 'ghost' | 'danger';
    size?: 'sm' | 'md' | 'lg';
    full?: boolean;
    loading?: boolean;
    disabled?: boolean;
    type?: 'button' | 'submit' | 'reset';
}>();

const emit = defineEmits<{ click: [e: MouseEvent] }>();

const base = 'inline-flex items-center justify-center gap-2 font-medium rounded-xl border transition-all duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed select-none cursor-pointer';

const variants: Record<string, string> = {
    primary: 'bg-primary text-primary-foreground border-transparent hover:opacity-90 active:scale-[0.98] shadow-sm',
    secondary: 'bg-surface-elevated text-foreground border-border hover:bg-surface hover:border-primary/40 active:scale-[0.98]',
    ghost: 'bg-transparent text-muted-foreground border-transparent hover:bg-surface hover:text-foreground',
    danger: 'bg-destructive text-destructive-foreground border-transparent hover:opacity-90 active:scale-[0.98]',
};

const sizes: Record<string, string> = {
    sm: 'h-8  px-3   text-xs  gap-1.5',
    md: 'h-10 px-4   text-sm',
    lg: 'h-12 px-6   text-base',
};

const classes = computed(() => [
    base,
    variants[props.variant ?? 'primary'],
    sizes[props.size ?? 'md'],
    props.full ? 'w-full' : '',
]);
</script>

<template>
    <button v-bind="$attrs" :type="type ?? 'button'" :disabled="disabled || loading" :class="classes"
        @click="emit('click', $event)">
        <!-- Loading spinner -->
        <span v-if="loading"
            class="w-4 h-4 rounded-full border-2 border-current border-t-transparent animate-spin shrink-0" />
        <slot />
    </button>
</template>
