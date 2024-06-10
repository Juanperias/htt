<template>
  <n-space vertical justify="end">

    <n-space justify="end" v-if="store.status == 200">
      <n-gradient-text type="success" :size="25">
        Status: {{ store.status }}
      </n-gradient-text>
    </n-space>
    <n-space v-else justify="end">
      <n-gradient-text :size="25" type="danger">
        Status: {{ store.status }}
      </n-gradient-text>
    </n-space>

    <n-space justify="end">
      <n-switch v-model:value="store.isJson">
        <template #checked>
          Json
        </template>
        <template #unchecked>
          Text
        </template>
      </n-switch>
    </n-space>


    <n-scrollbar style="max-height: 110px;" trigger="none">
      <div v-if="store.isJson == true">
        <n-gradient-text :size="24" type="warning">
          {{ store.response }}
        </n-gradient-text>
      </div>

      <div v-else>
        <n-gradient-text :gradient="{
          from: 'rgb(85, 85, 85)',
          to: 'rgb(170, 170, 170)'
        }">
          {{ store.response }}
        </n-gradient-text>
      </div>
    </n-scrollbar>
  </n-space>
</template>

<script setup lang="ts">
import { useResponseStore } from '../stores/responseStore';

const store = useResponseStore()

</script>
