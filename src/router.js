
import  HelloWorld  from './components/HelloWorld.vue'
import Chat from './views/Chat.vue'
import {createRouter, createWebHistory} from 'vue-router'

const routes = [
    { path: '/helloworld', name: 'HelloWorld', component: HelloWorld },
    { path: '/', name: 'chat', component: Chat }
]
const router = createRouter({
    history: createWebHistory(),
    routes,
  })

export default router;