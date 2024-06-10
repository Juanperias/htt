<template>
  <n-config-provider :theme="darkTheme">
    <div class="m-7">
      <RequestForm />
      <SetRequestProps />
      <WatchResponse />
    </div>
  </n-config-provider>
</template>

<script setup lang="ts">
import RequestForm from './components/RequestForm.vue'
import SetRequestProps from './components/SetRequestProps.vue';
import WatchResponse from './components/WatchResponse.vue'
import { darkTheme } from 'naive-ui'
</script>
