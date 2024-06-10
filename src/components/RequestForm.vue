<template>
  <div class="flex gap-5">
    <n-input type="text" v-model:value="requestStore.url" :maxlength="1000" placeholder="https://example.com" />
    <n-select v-model:value="requestStore.method" :options="methods" />
    <n-button @click="makeRequest" type="primary">
      Go!
    </n-button>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { useResponseStore } from '../stores/responseStore';
import { IResponse } from '../types/iResponse';
import { useRequestStore } from '../stores/requestStore';

const responseStore = useResponseStore()
const requestStore = useRequestStore()

const methods = [
  {
    label: 'GET',
    value: 'GET'
  },
  {
    label: 'POST',
    value: 'POST'
  },
  {
    label: 'PUT',
    value: 'PUT'
  },
  {
    label: 'DELETE',
    value: 'DELETE'
  },
  {
    label: 'PATCH',
    value: 'PATCH'
  }
]

async function makeRequest() {
  const response: IResponse = await invoke('make_request', {
    url: requestStore.url,
    headers: requestStore.headers,
    body: requestStore.body,
    method: requestStore.method
  })

  responseStore.setResponse(response.body)
  responseStore.setStatus(response.status)
}
</script>
