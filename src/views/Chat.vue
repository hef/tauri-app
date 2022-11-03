
<script setup>
import LogLine from '../components/LogLine.vue';
import ChatInput from '../components/ChatInput.vue';
import { useMessageStore } from '../stores/message';
import { listen } from '@tauri-apps/api/event'
const messages = useMessageStore();

listen("app://message", e => {
  // todo: switch to typescript.  I have too many things called "message" and it's confusing
  messages.addMessage({ id: 1,  message: JSON.parse(e.payload).data})
})
</script>

<style scoped>
.chatbox {
    height: 100vh;
}
</style>


<template>
    <div class="chatbox column reverse no-wrap">
        <div class="row">
            <div class="col">
                <ChatInput></ChatInput>
            </div>
        </div>
        <div class="row">
            <div class="col">
                <LogLine v-for="message in messages.messages" :key="message.id" :message="message.message"></LogLine>
            </div>
        </div>
</div>
</template>
