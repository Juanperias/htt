<template>
  <div class="mt-4">
    <n-input type="textarea" size="large" style="width: 100%; resize: false;"
      placeholder="type your request body! (IN JSON)" v-model:value="store.body" :minlength="300" round clearable />
  </div>
</template>

<script setup lang="ts">
import { useRequestStore } from '../stores/requestStore';


const store = useRequestStore()

</script>
