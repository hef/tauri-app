<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'


defineProps({
  msg: String
})

const count = ref(0)
const rust_count = ref(0)
const rust_bumped_count = ref(0)

function bumpCount() {
  invoke('bump_counter').then((value) => {rust_count.value = value})
}

listen("app://count", e => {
  rust_bumped_count.value = e.payload;
})

</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="count++">vue count is {{ count }}</button>
    <button type="button" @click="bumpCount()">rust count is {{ rust_count }}</button>
    <div>rust counter: {{ rust_bumped_count }}</div>
  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
