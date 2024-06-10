import { defineStore } from 'pinia'
import { IHeader } from '../types/IHeader';

export const useRequestStore = defineStore('requestStore', {
  state: (): { url: string, method: string, headers: IHeader[], body: string } => ({
    url: '',
    method: 'GET',
    headers: [{ name: "Example_header", value: "Example value" }],
    body: ''
  }),
  actions: {
    setUrl(newURl: string) {
      this.url = newURl
    },
    setMethod(newMethod: string) {
      this.method = newMethod
    },
    addEmptyHeader() {
      this.headers.push({
        name: "",
        value: ""
      })
    },
    removeHeader() {
      this.headers.splice(-1)
    },
    setBody(newBody: string) {
      this.body = newBody
    }
  }
})

