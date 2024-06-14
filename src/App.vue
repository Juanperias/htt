<template>
  <n-config-provider :theme="darkTheme">
    <main class="h-full">
      <RequestForm />
      <section class="grid grid-cols-[auto_minmax(auto,1fr)] w-full h-full justify-center gap-4">
        <SetRequestProps />
        <WatchResponse />
      </section>
    </main>
  </n-config-provider>

</template>

<script setup lang="ts">


import RequestForm from './components/RequestForm.vue'
import SetRequestProps from './components/SetRequestProps.vue';
import WatchResponse from './components/WatchResponse.vue'
import { darkTheme } from 'naive-ui'
</script>
