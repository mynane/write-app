import { createApp } from 'vue'
import { createPinia } from 'pinia'
import i18n from './locals'
import router from './router'
import App from './App.vue'
import VConsole from 'vconsole'

import '~/styles/index.scss'
import 'uno.css'

// If you want to use ElMessage, import it.
import 'element-plus/theme-chalk/src/message.scss'
import 'element-plus/theme-chalk/src/message-box.scss'
import 'element-plus/theme-chalk/src/affix.scss'
import 'element-plus/theme-chalk/src/card.scss'

import loadIcon from './utils/loadIcon'

new VConsole()
const pinia = createPinia()

const app = createApp(App)
loadIcon(app)

app.use(i18n)
app.use(pinia)
app.use(router)

app.mount('#app')
