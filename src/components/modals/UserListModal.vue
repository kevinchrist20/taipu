<script setup lang="ts">
import type { User } from '../../types'
import Modal from './Modal.vue'

interface Props {
  show: boolean
  users: User[]
}

defineProps<Props>()

const emit = defineEmits<{
  'close': []
  'select-user': [user: User]
}>()
</script>

<template>
  <Modal :show="show" size="lg" @close="emit('close')">
    <div class="text-center">
      <h2 class="text-3xl font-bold text-white mb-5">
        Select User
      </h2>
    </div>
    <div class="grid grid-cols-1 gap-4">
      <button
        v-for="user in users" :key="user.id" class="font-semibold rounded-lg py-3 px-5 shadow-md bg-gray-700 text-white transition-transform transform hover:bg-gray-600 hover:scale-105"
        @click="emit('select-user', user)"
      >
        <span class="capitalize">{{ user.name }}</span>
      </button>
    </div>
  </Modal>
</template>

<style scoped></style>
