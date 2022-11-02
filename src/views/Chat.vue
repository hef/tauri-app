
<script setup>
import LogLine from '../components/logline.vue';
import ChatInput from '../components/chatinput.vue';
import { useMessageStore } from '../stores/message';
import { listen } from '@tauri-apps/api/event'

const messages = useMessageStore();

listen("app://message", e => {
  messages.addMessage({ id: 1,  message: e.payload})
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
