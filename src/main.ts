import { createApp } from 'vue'
import { createPinia } from 'pinia'
import i18n from './locals'
import router from './router'
import App from './App.vue'

import '~/styles/index.scss'
import 'uno.css'

// If you want to use ElMessage, import it.
import 'element-plus/theme-chalk/src/message.scss'
import loadIcon from './utils/loadIcon'

const pinia = createPinia()

const app = createApp(App)
loadIcon(app)

app.use(i18n)
app.use(pinia)
app.use(router)

app.mount('#app')
