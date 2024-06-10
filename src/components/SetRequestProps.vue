<template>
  <n-select class="mt-4" v-model:value="modifyHeadersOrBody" size="small" :options="options" />
  <div v-if="modifyHeadersOrBody == 'HEADERS'">
    <SetHeaders />
  </div>
  <div v-else>
    <SetBody />
  </div>
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
