import { defineStore } from 'pinia'

export const useResponseStore = defineStore('responseStore', {
  state: () => ({
    response: '{ "message": "Hello!" }',
    status: 200,
    isJson: true
  }),
  actions: {
    setResponse(newResponse: string) {
      this.response = newResponse
    },
    setStatus(newStatus: number) {
      this.status = newStatus
    },
    setIsJson(newIsJson: boolean) {
      this.isJson = newIsJson
    }
  }
})
