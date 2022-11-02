import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { Quasar } from 'quasar'
import 'quasar/src/css/index.sass'
import { createPinia } from 'pinia'


import router  from './router.js'

createApp(App)
.use(router)
.use(Quasar, {
    Plugins: {},
    config: {}
})
.use(createPinia())
.mount('#app')
