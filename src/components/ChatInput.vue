
<script setup>
import { useMessageStore } from '../stores/message';
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
const text = ref("")
const messages = useMessageStore();


function submit() {
    const m = {id: 1, message: text.value}
    messages.addMessage(m)
    console.log(m)
    invoke('send_message', {message: text.value} ).then(() => { text.value = ""} )
}

</script>

<template>
    <div>
        <q-input outlined v-model="text" @keyup.enter="submit()"></q-input>
    </div>
</template>


<style scoped>

</style>