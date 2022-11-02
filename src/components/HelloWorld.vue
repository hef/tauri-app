<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'


defineProps({
  msg: String
})

const incoming_message = ref("")
const text = ref("")

listen("app://message", e => {
  incoming_message.value = e.payload
})

function submit() {
  invoke('send_message', {message: text.value} ).then(() => { text.value = ""} )
}

</script>

<template>
  <h1>{{ msg }}</h1>
  

  <div class="card">
    {{ incoming_message }}
  </div>
  <div>
      <input type="text" v-model=text @keyup.enter="submit()">
  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
