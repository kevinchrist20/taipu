<script lang="ts" setup>
import { watch } from 'vue';
import useAlert from '../../utils/useAlert';

const {
    alerts,
    currentAlert,
    showToast,
    clearCurrentAlert,
} = useAlert()
let timeOut: number | undefined

function clearAlert() {
    clearCurrentAlert()
    clearTimeout(timeOut as number)
} 

watch(currentAlert, () => {
    if (timeOut) {
        clearTimeout(timeOut)
        timeOut = undefined
    }
    if (currentAlert.value?.explicit)
        return
    timeOut = setTimeout(() => clearAlert(), currentAlert.value?.timer ?? 5000)
})
</script>

<template>
    <div v-if="showToast"
        class="toast-wrapper left-0 fixed top-[1%] flex items-center justify-center w-screen font-medium text-sm">
        <div class="flex justify-end items-center w-full pe-5 relative">
            <div v-for="(alert, index) in alerts" :key="alert?.position! + index"
                class="pointer shadow-sm p-4 rounded-md items-center transform transition-all absolute animate-slide-in-right animate-slide-in-right fw-400 animate-duration-60 ease-in-out top-[1rem] w-[22rem] border-zinc-1 border-1"
                :class="[
                    `notif-${alert?.type}`,
                ]">
                <div class="fw-medium">
                    <h6 v-if="currentAlert?.title" class="mb-1 font-semibold" :class="{
                        'danger-bg': alert.type === 'danger',
                        'success-bg': alert.type === 'success',
                        'warning-bg': alert.type === 'warning',
                        'info-bg': alert.type === 'info',
                    }">
                        {{ currentAlert.title }}
                    </h6>
                    <div class="grid grid-cols-7 items-end text-left text-sm" :class="currentAlert?.explicit && 'mb-2'">
                        <span class="col-span-6" :class="currentAlert?.title && 'text-zinc-5 @dark:text-zinc-3'">
                            {{ alert?.message }}
                        </span>

                        <div class="flex justify-end">
                            <div v-if="!alert.explicit" class="rounded-full cursor-pointer" :class="{
                                'hover:bg-red-500 danger-prose': alert.type === 'danger',
                                'hover:bg-amber-500 warning-prose': alert.type === 'warning',
                                'hover:bg-sky-500 info-prose': alert.type === 'info',
                                'hover:bg-emerald-500 success-prose': alert.type === 'success',
                            }" @click="clearAlert">
                                <div class="i-ph-x-bold" />
                            </div>
                        </div>
                    </div>
                    <div class="w-full flex justify-end">
                        <ui-btn v-if="currentAlert?.explicit" label="Close" :class="{
                            'danger-bg': alert.type === 'danger',
                            'success-bg': alert.type === 'success',
                            'warning-bg': alert.type === 'warning',
                            'info-bg': alert.type === 'info',
                        }" @click="clearAlert" />
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.toast-wrapper {
    z-index: 5000 !important;
}

.slide-up-enter-active,
.slide-up-leave-active {
    transition: all 0.3s;
}

.slide-up-enter-from,
.slide-up-leave-to {
    opacity: 0;
    transform: translateY(-1rem);
}
</style>
