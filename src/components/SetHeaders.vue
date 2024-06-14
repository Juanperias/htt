<template>
  <n-scrollbar class="max-h-80 w-full h-full" style="max-height: 110px;" trigger="none">
    <div v-for="(input, index) in store.headers" :key="index" class="flex gap-4 w-full mt-4">
      <n-input class="w-full" type="text" placeholder="Header name" v-model:value.trim="input.name" />
      <n-input class="w-full" type="text" placeholder="Header value" v-model:value="input.value" />
    </div>
  </n-scrollbar>
  <n-space class="mt-2">
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
