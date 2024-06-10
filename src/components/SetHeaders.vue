<template>
  <n-scrollbar style="max-height: 200px;" trigger="none" class="mt-4 mb-4">
    <n-space v-for="(input, index) in store.headers" :key="index" class="mt-4">
      <n-input type="text" placeholder="Header name" v-model:value.trim="input.name" />
      <n-input type="text" placeholder="Header value" v-model:value="input.value" />
    </n-space>
  </n-scrollbar>
  <n-space>
    <n-button strong secondary round type="primary" @click="store.addEmptyHeader">
      Add header
    </n-button>
    <n-button strong secondary round type="error" @click="store.removeHeader">
      Remove header
    </n-button>
  </n-space>
</template>

<script setup lang="ts">
import { useRequestStore } from '../stores/requestStore';

const store = useRequestStore()
</script>
