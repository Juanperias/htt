<template>
  <n-config-provider :hljs="hljs">
    <n-code language="json" :code="props.code"></n-code>
  </n-config-provider>
</template>
  
<script setup lang="ts">
import { defineProps } from 'vue'
import hljs from 'highlight.js/lib/core'
import json from 'highlight.js/lib/languages/json'
  
hljs.registerLanguage('json', json)

const props = defineProps(['code'])
</script>