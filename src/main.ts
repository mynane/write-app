import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import App from './App.vue'
import { createPinia } from 'pinia'

import 'element-plus/dist/index.css'
import './style.css'
const pinia = createPinia()

const app = createApp(App)
app.use(ElementPlus)
app.use(pinia)
app.mount('#app')
