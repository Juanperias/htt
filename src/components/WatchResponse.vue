<template>
  <n-space class="w-full px-8" vertical justify="end">
    <div class="flex justify-between w-full">
      <div class="flex items-center">
        <n-switch v-model:value="store.isJson">
          <template #checked>
            Json
          </template>
          <template #unchecked>
            Text
          </template>
        </n-switch>
      </div>
  
      <n-space justify="end" v-if="store.status == 200">
        <p class="text-lg text-white">
          Status: <span
            class="font-light text-green-500 bg-[#8FFF8F50] px-2 py-1 rounded-md"
            >{{ store.status }}</span>
        </p>
      </n-space>
      <n-space v-else justify="start">
        <p class="text-lg text-white">
          Status: <span
            class="font-light text-red-500 bg-[#FF6F6F50] px-2 py-1 rounded-md"
            >{{ store.status }}</span>
        </p>
      </n-space>
    </div>

    <n-scrollbar class="h-full" trigger="none">
      <div v-if="store.isJson == true">
        <JsonResponse :code="store.response"/>
      </div>

      <div v-else>
        <p class="text-base text-white">
          {{ store.response }}
        </p>
      </div>
    </n-scrollbar>
  </n-space>
</template>

<script setup lang="ts">
import { useResponseStore } from '../stores/responseStore';
import JsonResponse from './JsonResponse.vue';

const store = useResponseStore()
</script>
