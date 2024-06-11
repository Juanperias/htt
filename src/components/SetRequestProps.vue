<template>
  <article class="w-full mt-4 px-8">
    <n-select v-model:value="modifyHeadersOrBody" size="small" :options="options" />

    <SetHeaders v-if="modifyHeadersOrBody == 'HEADERS'" />
    <SetBody v-else />
  </article>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import SetHeaders from './SetHeaders.vue';
import SetBody from './SetBody.vue';

const modifyHeadersOrBody = ref("HEADERS")


const options = [{
  label: "headers",
  value: "HEADERS",
}, {
  label: "body",
  value: "BODY"
}]


</script>
