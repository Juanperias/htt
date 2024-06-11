<template>
  <section class="flex mx-8 my-4 gap-5 px-3 py-4 bg-zinc-900 rounded-lg">
    <n-select
      class="max-w-40"
      v-model:value="requestStore.method"
      :options="methods" 
    />
    <n-input 
      type="text"
      :round="true"
      v-model:value="requestStore.url"
      :maxlength="1000"
      placeholder="https://example.com"
    />
    <n-button class="max-w-24 w-full" :round="true" @click="makeRequest" type="primary">
      Go!
    </n-button>
  </section>
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
