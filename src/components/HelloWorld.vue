<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'


defineProps({
  msg: String
})


const count = ref(0)
const rust_count = ref(0)


function bumpCount() {
  invoke('bump_counter').then((value) => {rust_count.value = value})
}

listen("foo", e => {
  console.log("foo", e.payload);
})

</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="count++">vue count is {{ count }}</button>
    <button type="button" @click="bumpCount()">rust count is {{ rust_count }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
  </div>

  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank"
      >create-vue</a
    >, the official Vue + Vite starter
  </p>
  <p>
    Install
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
    in your IDE for a better DX
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
