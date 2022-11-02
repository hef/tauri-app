<script setup>
import { useQuasar } from 'quasar'
import { ref } from 'vue'

const $q = useQuasar()
$q.dark.set(true)

const leftDrawerOpen = ref(true)

</script>

<template>
  <div id="q-app"></div>
  <q-layout view="hHh lpR fFf">

    <q-drawer show-if-above v-model="leftDrawerOpen" side="left" behavior="desktop" bordered>
      <!-- drawer content -->
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>

  </q-layout>
</template>

<style scoped>
.q-app{
  min-height: 100vh;
}
</style>
