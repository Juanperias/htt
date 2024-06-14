import { defineStore } from 'pinia'

export const useResponseStore = defineStore('responseStore', {
  state: () => ({
    response: 'Welcome to htt!',
    status: 200,
    isJson: false,
    contentType: "text/html"
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
    },
    setContentType(newContentType: string) {
      this.contentType = newContentType
    }
  }
})
