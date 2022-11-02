import { defineStore } from 'pinia'

export const useMessageStore = defineStore('messages',{
    state: () => ({
        messages: [],
    }),
    actions: {
        addMessage(message) {
            this.messages.push(message)

        },
        clearMessages() {
            this.messages = []
        }
    }
})